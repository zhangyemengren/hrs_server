use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::PgPool;

#[derive(serde::Serialize)]
pub struct Module {
    id: i32,
    module_type: String,
}

#[derive(serde::Serialize)]
pub struct ModuleResponse {
    total: usize,
    data: Vec<Module>,
}

pub async fn get_modules(State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query_as!(Module, "SELECT id, type AS module_type FROM modules")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(ModuleResponse {
        total: result.len(),
        data: result,
    })
}