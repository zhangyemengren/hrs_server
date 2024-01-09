use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::{CookieJar, PrivateCookieJar};
use cookie::{time::Duration, Cookie};

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

pub async fn get_modules(State(AppState { pool, .. }): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as!(Module, "SELECT id, type AS module_type FROM modules")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(ModuleResponse {
        total: result.len(),
        data: result,
    })
}

pub async fn login(jar_private: PrivateCookieJar, jar: CookieJar) -> impl IntoResponse {
    let duration = Duration::minutes(60);
    let cookie_private = Cookie::build(("session", "this is a session"))
        .path("/")
        .http_only(true)
        .max_age(duration)
        .build();
    let cookie = Cookie::build(("is_login", "1")).path("/").build();
    (
        jar_private.add(cookie_private),
        jar.add(cookie),
        StatusCode::OK,
    )
}
