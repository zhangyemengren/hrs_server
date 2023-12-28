use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use hrs_server::startup::{spawn, App};
use sqlx::PgPool;
use tower::ServiceExt;

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
    let app = App::new().with_router().await.app;

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "Hello, World!");
}

#[tokio::test]
async fn test_health_check() {
    let app = App::new().with_router().await.app;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "");
}

#[tokio::test]
async fn test_not_found() {
    let app = App::new().with_router().await.app;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/not_found")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "<h1>404</h1>");
}
