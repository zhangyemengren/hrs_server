use serde::Serialize;

#[derive(Serialize)]
pub struct GenericBody<T>
where
    T: Serialize,
{
    pub code: u16,
    pub msg: String,
    pub data: T,
}
