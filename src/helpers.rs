use axum::body::Body;
use axum::http;
use axum::http::{header, Request};
use tower::ServiceExt;
use crate::startup::App;

pub async fn do_login() -> String {
    let app = App::new().with_router().await.app;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/login")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    cookie
}