use crate::response::GenericBody;
use axum::Json;
use axum::response::IntoResponse;

pub async fn get_user() -> impl IntoResponse {
    Json(GenericBody {
        code: 200,
        msg: "success".to_string(),
        data: "hello".to_string()
    })
}
