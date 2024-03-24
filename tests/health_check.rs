pub mod helpers;
use axum::{
    body::{to_bytes, Bytes},
    http::StatusCode,
};
use helpers::do_request;
use hrs_server::{
    response::{GenericBody, Status},
    startup::spawn,
};
use sqlx::PgPool;

#[sqlx::test]
async fn test_db_link() {
    spawn().await.expect("Spawn server failed");
    let pool = PgPool::connect("postgres://postgres:qwer1234@localhost:5432/hrs")
        .await
        .unwrap();
    let rec = sqlx::query!("SELECT * FROM companies")
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("{:?}", rec);
    assert!(true);
}
// 可以使用tower而不生成HTTP server:
#[tokio::test]
async fn test_root() {
    let response = do_request("/", "", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "Hello, World!");
}

#[tokio::test]
async fn test_health_check() {
    let response = do_request("/health_check", "", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "");
}

#[tokio::test]
async fn test_not_found() {
    let response = do_request("/not_found", "", None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::to_vec(&GenericBody {
        status: Status::HttpError,
        msg: "Not Found".to_string(),
        data: "".to_string(),
    })
    .unwrap();
    let value = Bytes::from(value);

    assert_eq!(body, value);
}
