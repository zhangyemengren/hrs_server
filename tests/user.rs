use axum::body::{to_bytes, Body};
use axum::http::{header, Request, StatusCode};
use hrs_server::startup::App;
use serde_json::{json, Value};
use tower::ServiceExt;
use hrs_server::helpers::do_login;
use hrs_server::response::GenericBody;

#[tokio::test]
async fn test_get_user() {
    let app = App::new().with_router().await.app;
    let cookie = do_login().await;
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
        code: 200,
        msg: "success".to_string(),
        data: "hello".to_string()
    });
    assert_eq!(body, value);
}
