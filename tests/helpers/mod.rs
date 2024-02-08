// 在tests文件夹中创建mod.rs文件不会添加到测试范围 用于编写一些测试辅助函数
use axum::body::Body;
use axum::http::{header, Method, Request};
use axum::response::Response;
use hrs_server::startup::App;
use serde::Serialize;
use serde_json::{json, Value};
use tower::ServiceExt;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
pub async fn do_login(user: User) -> Response {
    let app = App::new().with_router().await.app;

    app.oneshot(
        Request::builder()
            .method(Method::POST)
            .uri("/api/login")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!(user).to_string()))
            .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn do_admin_login() -> Response {
    let app = App::new().with_router().await.app;

    app.oneshot(
        Request::builder()
            .method(Method::POST)
            .uri("/api/login")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                json!(User {
                    username: "admin".to_string(),
                    password: "admin".to_string(),
                })
                .to_string(),
            ))
            .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn do_request(uri: &str, token: &str, body: Option<Body>) -> Response {
    let body = body.unwrap_or(Body::empty());
    let app = App::new().with_router().await.app;
    app.oneshot(
        Request::builder()
            .uri(uri)
            .header(header::AUTHORIZATION, token)
            .body(body)
            .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn get_data(response: Response) -> Value {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&body).unwrap()
}

pub async fn get_string(response: Response, key: &str) -> String {
    let data: Value = get_data(response).await;
    data.get(key).unwrap().as_str().unwrap().to_string()
}
