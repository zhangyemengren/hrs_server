// https://users.rust-lang.org/t/invalid-dead-code-warning-for-submodule-in-integration-test/80259
// 没有完全使用helpers导出的函数会发出#[warn(dead_code)]警告 因为每一个测试文件都会引用一个mod helpers 副本
// 减少集成测试文件数可以防止编译多个公共依赖副本 减少编译体积
pub mod helpers;
use axum::{
    body::{to_bytes, Bytes},
    http::StatusCode,
};
use helpers::{do_admin_login, do_login, do_request, get_string, User};
use hrs_server::response::{GenericBody, Status};
use serde_json::{from_slice, Value};

#[tokio::test]
async fn test_get_user() {
    let response = do_admin_login().await;
    let token = get_string(response, "data").await;
    let response = do_request("/api/users", &token, None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::to_vec(&GenericBody {
        status: Status::Success,
        msg: "success".to_string(),
        data: "hello".to_string(),
    })
    .unwrap();
    let value = Bytes::from(value);
    assert_eq!(body, value);
}

#[tokio::test]
async fn test_login() {
    // 成功
    let response = do_login(User {
        username: "admin".to_string(),
        password: "admin".to_string(),
    })
    .await;
    let token = get_string(response, "data").await;
    assert!(token.len() > 0);
    // UsernameEmpty
    let response = do_login(User {
        username: "".to_string(),
        password: "admin".to_string(),
    })
    .await;
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::to_vec(&GenericBody {
        status: Status::Fail("UsernameEmpty".to_string()),
        msg: "".to_string(),
        data: (),
    })
    .unwrap();
    let value = Bytes::from(value);
    assert_eq!(body, value);
    // PasswordEmpty
    let response = do_login(User {
        username: "admin".to_string(),
        password: "".to_string(),
    })
    .await;
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::to_vec(&GenericBody {
        status: Status::Fail("PasswordEmpty".to_string()),
        msg: "".to_string(),
        data: (),
    })
    .unwrap();
    let value = Bytes::from(value);
    assert_eq!(body, value);
    // UsernameOrPasswordFormatError
    let response = do_login(User {
        username: "ab".to_string(),
        password: "123".to_string(),
    })
    .await;
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::to_vec(&GenericBody {
        status: Status::Fail("UsernameOrPasswordFormatError".to_string()),
        msg: "".to_string(),
        data: (),
    })
    .unwrap();
    let value = Bytes::from(value);
    assert_eq!(body, value);
    // UsernameOrPasswordError
    let response = do_login(User {
        username: "bob@123.com".to_string(),
        password: "123xvbM!".to_string(),
    })
    .await;
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::to_vec(&GenericBody {
        status: Status::Fail("UsernameOrPasswordError".to_string()),
        msg: "".to_string(),
        data: (),
    })
    .unwrap();
    let value = Bytes::from(value);
    assert_eq!(body, value);
}

#[tokio::test]
async fn test_get_modules() {
    // admin
    let response = do_admin_login().await;
    let token = get_string(response, "data").await;
    let response = do_request("/api/modules", &token, None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = from_slice(&body).unwrap();
    let len = json.get("data").unwrap().as_array().unwrap().len();
    assert_eq!(len, 8);
    //  员工
    let response = do_login(User {
        username: "bob@qq.com".to_string(),
        password: "Qwer1234!".to_string(),
    })
    .await;
    let token = get_string(response, "data").await;
    let response = do_request("/api/modules", &token, None).await;
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = from_slice(&body).unwrap();
    let len = json.get("data").unwrap().as_array().unwrap().len();
    assert_eq!(len, 7);
    // 求职者
    let response = do_login(User {
        username: "david@gmail.com".to_string(),
        password: "abcABC123!".to_string(),
    })
    .await;
    let token = get_string(response, "data").await;
    let response = do_request("/api/modules", &token, None).await;
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = from_slice(&body).unwrap();
    let len = json.get("data").unwrap().as_array().unwrap().len();
    assert_eq!(len, 1);
}
