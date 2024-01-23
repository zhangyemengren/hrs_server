// https://users.rust-lang.org/t/invalid-dead-code-warning-for-submodule-in-integration-test/80259
// 没有完全使用helpers导出的函数会发出#[warn(dead_code)]警告 因为每一个测试文件都会引用一个mod helpers 副本
// 减少集成测试文件数可以防止编译多个公共依赖副本 减少编译体积
mod helpers;
use axum::body::{to_bytes, Body};
use axum::http::{header, Request, StatusCode};
use helpers::{do_admin_login, do_login, User};
use hrs_server::response::{GenericBody, Status};
use hrs_server::startup::App;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn test_get_user() {
    let app = App::new().with_router().await.app;
    let cookie = do_admin_login().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/users")
                .header(header::COOKIE, cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");
    let value = json!(GenericBody {
        status: Status::Success,
        msg: "success".to_string(),
        data: "hello".to_string()
    });
    assert_eq!(body, value);
}

#[tokio::test]
async fn test_login() {
    let cookie = do_login(User {
        username: "admin".to_string(),
        password: "admin".to_string(),
    })
    .await;
    assert!(cookie.len() > 0);

    // let cookie = do_login(User{
    //     username: "admin".to_string(),
    //     password: "admin1".to_string(),
    // }).await;
    // assert_eq!(cookie.len(), 0);
}
