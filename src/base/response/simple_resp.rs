use serde::Serialize;
use crate::base;
use crate::base::{BizError, StatusCode};

#[derive(Serialize)]
pub struct SimpleResponse<T: serde::Serialize> {
    pub data: Option<T>,
    pub status: StatusCode,
    pub msg: String,
}

impl<T: serde::Serialize> actix_web::Responder for SimpleResponse<T> {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&self).unwrap())
    }
}

impl <T: serde::Serialize> SimpleResponse<T> {
    pub fn success(data: Option<T>) -> SimpleResponse<T> {
        SimpleResponse {
            data,
            status: StatusCode::Success,
            msg: "success".to_string(),
        }
    }
    pub fn fail(err: BizError) -> SimpleResponse<T> {
        SimpleResponse {
            data: None,
            status: err.code,
            msg: err.msg,
        }
    }
}
