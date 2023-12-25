use crate::configuration::{get_config, Configuration};
use crate::routes::{api::get_user, catch_all, health_check, root};
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct App {
    pub app: Router,
    pub config: Configuration,
}

impl App {
    pub fn new() -> Self {
        Self {
            app: Self::get_router(),
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

    // pub fn build(self) -> Self{
    //     self.effect_with_log()
    // }

    pub async fn get_listener(&self) -> Result<tokio::net::TcpListener, anyhow::Error> {
        let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            self.config.application.host, self.config.application.port
        ))
        .await?;
        Ok(listener)
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let listener = self.get_listener().await?;
        println!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, self.app).await?;
        Ok(())
    }

    pub async fn spawn(self) -> Result<(), anyhow::Error> {
        let listener = self.get_listener().await?;
        tokio::spawn(async move {
            axum::serve(listener, self.app).await.unwrap();
        });
        Ok(())
    }
    fn get_router() -> Router {
        let api_v1 = Router::new().route("/users", get(get_user));
        let api = Router::new().nest("/v1", api_v1);
        let routers = Router::new()
            .route("/", get(root))
            .route("/health_check", get(health_check))
            .route("/*all", get(catch_all))
            .nest("/api", api);
        routers
    }
}
