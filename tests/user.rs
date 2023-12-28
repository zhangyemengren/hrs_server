use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use hrs_server::startup::App;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_user() {
    let app = App::new().with_router().await.app;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = json!({
        "data":[
            {
                "id": 1,
                "name": "John Doe",
            },
            {
                "id": 2,
                "name": "Miles Davis",
            },
        ],
        "total": 2,
    });
    assert_eq!(body, value.to_string());
}
