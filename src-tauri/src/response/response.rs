use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub data: T,
    pub message: String,
}

impl<T> Response<T> {
    pub fn new(code: i32, data: T, message: String) -> Response<T> {
        Response {
            code,
            data,
            message: message.to_string(),
        }
    }
}
