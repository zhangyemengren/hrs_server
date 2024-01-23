use crate::response::{GenericBody, Status};
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};

pub async fn root() -> &'static str {
    "Hello, World!"
}
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn catch_all() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(GenericBody {
            status: Status::HttpError,
            msg: "Not Found".to_string(),
            data: "".to_string(),
        }),
    )
}
