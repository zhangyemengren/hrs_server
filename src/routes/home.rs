use axum::http::StatusCode;
use axum::response::Html;

pub async fn root() -> &'static str {
    "Hello, World!"
}
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn catch_all() -> (StatusCode, Html<&'static str>) {
    (StatusCode::NOT_FOUND, Html("<h1>404</h1>"))
}
