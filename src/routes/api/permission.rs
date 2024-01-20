use crate::{startup::AppState, Jwt};
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
    icon_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ModuleResponse {
    total: usize,
    data: Vec<Module>,
}

pub async fn get_modules(State(AppState { pool, .. }): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Module,
        "SELECT id, type AS module_type, icon_url FROM modules ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(ModuleResponse {
        total: result.len(),
        data: result,
    })
}

pub async fn login(
    State(AppState { pool, .. }): State<AppState>,
    jar_private: PrivateCookieJar,
    jar: CookieJar,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "SELECT u.*
FROM users u
         JOIN user_credentials uc ON u.id = uc.user_id
WHERE uc.username = 'admin' AND uc.password = 'admin';"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    println!("{:?}", result);
    let duration = Duration::minutes(60);
    let token = Jwt::default().new_token().unwrap();
    let cookie_private = Cookie::build(("token", token))
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

pub async fn logout(jar_private: PrivateCookieJar, jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build("is_login").path("/").build();
    let cookie_token = Cookie::build("token").path("/").build();
    (
        jar.remove(cookie),
        jar_private.remove(cookie_token),
        StatusCode::OK,
    )
}
