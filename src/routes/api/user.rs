use crate::response::{GenericBody, Status};
use axum::response::IntoResponse;
use axum::Json;

pub async fn get_user() -> impl IntoResponse {
    Json(GenericBody {
        status: Status::Success,
        msg: "success".to_string(),
        data: "hello".to_string(),
    })
}
