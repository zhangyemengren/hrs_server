use crate::{
    auth::{Claims, Jwt, Permission, UserInfo},
    response::{ApiError, GenericBody, Status},
    startup::AppState,
    Validator,
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Serialize)]
struct Module {
    id: i32,
    module_type: String,
}

#[derive(Debug, Serialize)]
struct ModuleResponse {
    total: usize,
    data: Vec<Module>,
}

pub async fn get_modules(
    Extension(claims): Extension<Claims>,
    State(AppState { pool, .. }): State<AppState>,
) -> impl IntoResponse {
    let permission_list: Vec<_> = claims
        .permission
        .iter()
        .map(|permission| permission.module_id)
        .collect();
    let result = sqlx::query_as!(
        Module,
        "SELECT id, type AS module_type FROM modules WHERE id = ANY($1) ORDER BY id",
        &permission_list
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(ModuleResponse {
        total: result.len(),
        data: result,
    })
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Debug, Deserialize)]
pub enum LoginValidateError {
    UsernameEmpty,
    PasswordEmpty,
    UsernameOrPasswordError,
    UsernameOrPasswordFormatError,
}

impl Display for LoginValidateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoginValidateError::UsernameEmpty => write!(f, "UsernameEmpty"),
            LoginValidateError::PasswordEmpty => write!(f, "PasswordEmpty"),
            LoginValidateError::UsernameOrPasswordError => write!(f, "UsernameOrPasswordError"),
            LoginValidateError::UsernameOrPasswordFormatError => {
                write!(f, "UsernameOrPasswordFormatError")
            }
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
        if !self.username_reg_validate(&self.username)
            || !self.password_reg_validate(&self.password)
        {
            return Err(LoginValidateError::UsernameOrPasswordFormatError);
        }
        Ok(())
    }
}

pub async fn login(
    State(AppState { pool, .. }): State<AppState>,
    WithRejection(Json(login_request), _): WithRejection<Json<LoginRequest>, ApiError>,
    // Json(login_request): Json<LoginRequest>,
) -> Response {
    let validate_result = validate_password(&pool, &login_request).await;
    if let Err(e) = validate_result {
        return Json(GenericBody {
            status: Status::Fail(e.to_string()),
            msg: "".to_string(),
            data: (),
        })
        .into_response();
    }
    let data = validate_result.unwrap();
    println!("data: {:?}", data);
    let token = Jwt::default()
        .user(data.0)
        .permission(data.1)
        .new_token()
        .unwrap();

    (Json(GenericBody {
        status: Status::Success,
        msg: "success".to_string(),
        data: token,
    }),)
        .into_response()
}

async fn validate_password(
    pool: &PgPool,
    payload: &LoginRequest,
) -> Result<(UserInfo, Vec<Permission>), LoginValidateError> {
    payload.validate()?;

    let user = sqlx::query_as!(
        UserInfo,
        "SELECT u.name, u.role, uc.user_id, uc.username FROM users u
        JOIN user_credentials uc ON u.id = uc.user_id
        WHERE uc.username = $1 AND uc.password = $2;",
        payload.username,
        payload.password
    )
    .fetch_one(pool)
    .await
    .map_err(|_| LoginValidateError::UsernameOrPasswordError)?;
    let permission = sqlx::query_as!(
        Permission,
        "SELECT module_id FROM permissions WHERE role_id = $1;",
        &user.role
    )
    .fetch_all(pool)
    .await
    .map_err(|_| LoginValidateError::UsernameOrPasswordError)?;
    Ok((user, permission))
}
