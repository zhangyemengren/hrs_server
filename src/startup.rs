use crate::configuration::{get_config, Configuration};
use crate::routes::api::get_modules;
use crate::routes::{api::get_user, catch_all, health_check, root};
use axum::extract::{self, FromRef, MatchedPath};
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::{middleware, Router};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
        let api_v1 = Router::new()
            .route("/users", get(get_user))
            .route("/modules", get(get_modules));
        let api = Router::new().nest("/v1", api_v1);
        let pool = self.get_pool().await;
        let state = AppState {
            pool,
            cookie_key: Key::generate(),
        };
        let routers = Router::new()
            .route("/", get(root))
            .route("/health_check", get(health_check))
            .route("/*all", get(catch_all))
            .nest("/api", api)
            .with_state(state.clone())
            .layer(
                ServiceBuilder::new().layer(middleware::from_fn_with_state(state.clone(), auth)),
            );
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

pub async fn auth(
    jar: PrivateCookieJar,
    request: extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // do something with `request`...

    let response = next.run(request).await;
    let data = jar.get("secret");
    println!("response: {:?}", data);
    // do something with `response`...
    Ok(response)
    // Err(StatusCode::UNAUTHORIZED)
}

// async fn get_secret(jar: PrivateCookieJar) {
//     if let Some(data) = jar.get("secret") {
//         // ...
//     }
// }
