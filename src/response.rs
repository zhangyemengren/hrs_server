use serde::ser::Serializer;
use serde::Serialize;

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
