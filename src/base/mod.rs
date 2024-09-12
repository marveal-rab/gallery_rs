pub mod response;

#[derive(serde::Serialize)]
pub enum StatusCode {
    Success = 0,
    Fail = 1,
}

#[derive(serde::Serialize)]
pub struct BizError {
    pub code: StatusCode,
    pub msg: String,
}

impl BizError {
    pub fn new_with_msg(code: StatusCode, msg: &str) -> Self {
        BizError {
            code,
            msg: msg.to_string(),
        }
    }

    pub fn new(code: StatusCode) -> Self {
        BizError {
            code,
            msg: "failed".to_string()
        }
    }
}