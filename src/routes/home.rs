use axum::response::Html;
use axum::Json;
use serde_json::{json, Value};

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn catch_all() -> Html<&'static str> {
    Html("<h1>404</h1>")
}
pub async fn test_user() -> Json<Value> {
    Json(json!({
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
    }))
}
