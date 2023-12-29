use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn get_user(State(_pool): State<PgPool>) -> Json<Value> {
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

pub async fn add_user(State(_pool): State<PgPool>) -> Json<Value> {
    Json(json!({
        "data": {
            "id": 3,
            "name": "John Coltrane",
        },
    }))
}
