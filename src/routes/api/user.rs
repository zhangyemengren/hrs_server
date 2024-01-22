use crate::response::GenericBody;
use axum::response::IntoResponse;
use axum::Json;

pub async fn get_user() -> impl IntoResponse {
    Json(GenericBody {
        code: 200,
        msg: "success".to_string(),
        data: "hello".to_string(),
    })
}
