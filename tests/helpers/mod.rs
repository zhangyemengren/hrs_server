// 在tests文件夹中创建mod.rs文件不会添加到测试范围 用于编写一些测试辅助函数
use axum::body::Body;
use axum::http::{header, Method, Request};
use axum::response::Response;
use hrs_server::startup::App;
use serde::Serialize;
use serde_json::json;
use tower::ServiceExt;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
pub async fn do_login(user: User) -> (Response, String) {
    let app = App::new().with_router().await.app;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!(user).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let cookie = get_cookie(&response);
    (response, cookie)
}

pub async fn do_admin_login() -> String {
    let app = App::new().with_router().await.app;

    let response = app
        .oneshot(
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
        .unwrap();
    get_cookie(&response)
}

fn get_cookie(response: &Response) -> String {
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    cookie
}
