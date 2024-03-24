use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde::{ser::Serializer, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Serialize)]
pub struct GenericBody<T>
where
    T: Serialize,
{
    pub status: Status,
    pub msg: String,
    pub data: T,
}

pub enum Status {
    Success,      // 业务成功
    HttpError,    // http code处理 不在额外说明
    Fail(String), // http code 200 但业务失败
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Status::Success => serializer.serialize_str("Success"),
            Status::HttpError => serializer.serialize_str("HttpError"),
            Status::Fail(ref reason) => serializer.serialize_str(reason),
        }
    }
}
// 改变axum默认提取器做错逻辑(返回422 400等) 转换为200返回
#[derive(Debug, Error)]
pub enum ApiError {
    // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
    // implementation. See `thiserror` docs for more information
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!(GenericBody {
            status: Status::Fail("参数错误".to_string()),
            msg: "参数错误".to_string(),
            data: (),
        });
        let code = match self {
            ApiError::JsonExtractorRejection(x) => match x {
                JsonRejection::JsonDataError(_) => StatusCode::OK,
                JsonRejection::JsonSyntaxError(_) => StatusCode::OK,
                JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };
        (code, Json(payload)).into_response()
    }
}
