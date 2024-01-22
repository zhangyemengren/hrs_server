// 在tests文件夹中创建mod.rs文件不会添加到测试范围 用于编写一些测试辅助函数
use axum::body::Body;
use axum::http::{header, Method, Request};
use hrs_server::startup::App;
use tower::ServiceExt;

pub async fn do_login() -> String {
    let app = App::new().with_router().await.app;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"username":"admin","password":"admin"}"#))
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