use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use hrs_server::startup::App;
use sqlx::PgPool;
use tower::ServiceExt;

async fn spawn_app() {
    let app = App::new().serve;

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    // let _ =tokio::spawn(server);
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
}

#[sqlx::test]
async fn test_db() {
    spawn_app().await;
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
async fn hello_world() {
    let app = App::new().serve;

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "<h1>Hello, World!</h1>");
}
