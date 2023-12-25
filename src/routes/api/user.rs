use axum::Json;
use serde_json::{json, Value};

pub async fn get_user() -> Json<Value> {
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
