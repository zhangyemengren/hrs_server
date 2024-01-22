use crate::{startup::AppState, Jwt, Validator};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::{CookieJar, PrivateCookieJar};
use cookie::Cookie;
use sqlx::PgPool;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(serde::Serialize)]
struct Module {
    id: i32,
    module_type: String,
    icon_url: Option<String>,
}

#[derive(serde::Serialize)]
struct ModuleResponse {
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

#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Debug)]
pub enum LoginValidateError {
    UsernameEmpty,
    PasswordEmpty,
}

impl Display for LoginValidateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoginValidateError::UsernameEmpty => write!(f, "Username cannot be empty"),
            LoginValidateError::PasswordEmpty => write!(f, "Email cannot be empty"),
        }
    }
}

impl Error for LoginValidateError {}

impl Validator for LoginRequest {
    type Failure = LoginValidateError;
    fn validate(&self) -> Result<(), Self::Failure> {
        if self.str_empty(&self.username) {
            return Err(LoginValidateError::UsernameEmpty);
        }
        if self.str_empty(&self.password) {
            return Err(LoginValidateError::PasswordEmpty);
        }
        Ok(())
    }
}

pub async fn login(
    State(AppState { pool, .. }): State<AppState>,
    jar_private: PrivateCookieJar,
    jar: CookieJar,
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {
    let has_user = validate_password(&pool, &login_request).await.unwrap();
    println!("has_user: {:?}", has_user);
    let token = Jwt::default().new_token().unwrap();
    let cookie_private = Cookie::build(("token", token))
        .path("/")
        .http_only(true)
        .build();
    let cookie = Cookie::build(("is_login", "1")).path("/").build();
    (
        jar_private.add(cookie_private),
        jar.add(cookie),
        StatusCode::OK,
    )
}

async fn validate_password(pool: &PgPool, payload: &LoginRequest) -> anyhow::Result<()> {
    payload.validate()?;

    let result = sqlx::query!(
        "SELECT u.*
FROM users u
         JOIN user_credentials uc ON u.id = uc.user_id
WHERE uc.username = 'admin' AND uc.password = 'admin';"
    )
    .fetch_all(pool)
    .await?;
    if result.len() == 0 {
        return Err(anyhow::anyhow!("xxxx"));
    }
    println!("{:?}", result);
    Ok(())
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
