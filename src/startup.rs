use crate::configuration::{get_config, Configuration};
use crate::middlewares::auth::auth as authFn;
use crate::routes::{catch_all, get_modules, get_user, health_check, login, logout, root};
use axum::extract::{FromRef, MatchedPath};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{header, Method, Request};
use axum::middleware::{self};
use axum::routing::{get, post};
use axum::Router;
use axum_extra::extract::cookie::Key;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const COOKIE_SECRET: &[u8] = b"c7d46123b9a9cceac5ac32b3359fc02a22fcfdb30c7050c385609c9390110078277352e4a52aea1d17c6c6a75b42a0f1c8a86b2e812d1abf2cb4924c7d0e1f5e";
/// 应用
pub struct App {
    pub app: Router<()>,
    pub config: Configuration,
}
/// 共享状态
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            app: Router::new(),
            config: get_config(),
        }
    }
    pub fn effect_with_log(self) -> Self {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    // axum logs rejections from built-in extractors with the `axum::rejection`
                    // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                    "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace"
                        .into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        let app = self.app.layer(TraceLayer::new_for_http().make_span_with(
            |request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            },
        ));
        Self { app, ..self }
    }

    async fn get_pool(&self) -> PgPool {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.config.database.username,
            self.config.database.password,
            self.config.database.host,
            self.config.database.port,
            self.config.database.db_name
        );
        let pool = PgPoolOptions::new()
            .max_connections(self.config.database.max_connections)
            .connect(&connection_string)
            .await
            .expect("Failed to connect to Postgres.");
        pool
    }

    // pub async fn build(self) -> Self{
    //     self.with_router().await.effect_with_log()
    // }

    pub async fn with_router(self) -> Self {
        let api = Router::new()
            .route("/users", get(get_user))
            .route("/modules", get(get_modules));
        let api_without_auth = Router::new()
            .route("/login", post(login))
            .route("/logout", get(logout));
        let pool = self.get_pool().await;
        let state = AppState {
            pool,
            cookie_key: Key::from(COOKIE_SECRET),
        };
        let server_dir = ServeDir::new("assets").not_found_service(catch_all.into_service());
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE])
            .allow_origin([
                "http://localhost:3000".parse().unwrap(),
                "http://127.0.0.1:3000".parse().unwrap(),
                "https://hrs-web-gamma.vercel.app".parse().unwrap(),
            ])
            .allow_credentials(true);
        let routers = Router::new()
            .nest("/api", api)
            .layer(
                ServiceBuilder::new().layer(middleware::from_fn_with_state(state.clone(), authFn)),
            )
            .route("/", get(root))
            .route("/health_check", get(health_check))
            .nest("/api", api_without_auth)
            .route("/*all", get(catch_all))
            .with_state(state.clone())
            .nest_service("/assets", server_dir)
            .layer(ServiceBuilder::new().layer(cors));
        Self {
            app: routers,
            ..self
        }
    }
}

async fn get_listener(host: &str, port: &str) -> Result<tokio::net::TcpListener, anyhow::Error> {
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
    Ok(listener)
}

pub async fn run() -> Result<(), anyhow::Error> {
    let app = App::new().with_router().await.effect_with_log();
    let listener = get_listener(&app.config.application.host, &app.config.application.port).await?;

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.app).await?;
    Ok(())
}

pub async fn spawn() -> Result<(), anyhow::Error> {
    let app = App::new().with_router().await;
    let listener = get_listener(&app.config.application.host, &app.config.application.port).await?;
    tokio::spawn(async move {
        axum::serve(listener, app.app).await.unwrap();
    });
    Ok(())
}
