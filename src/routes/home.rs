use crate::response::{GenericBody, Status};
use axum::{http::StatusCode, response::IntoResponse, Json};

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
