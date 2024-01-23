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

#[derive(Serialize)]
pub enum Status {
    Success,      // 业务成功
    HttpError,    // http code处理 不在额外说明
    Fail(String), // http code 200 但业务失败
}
