use crate::base::{BizError, StatusCode};

#[derive(serde::Serialize)]
pub struct ListResponse<T: serde::Serialize> {
    pub data: Option<Vec<T>>,
    pub total: Option<u32>,
    pub status: StatusCode,
    pub msg: String,
}

impl<T: serde::Serialize> actix_web::Responder for ListResponse<T> {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&self).unwrap())
    }
}

impl <T: serde::Serialize> ListResponse<T> {
    pub fn success(data: Vec<T>, total: u32) -> ListResponse<T> {
        ListResponse {
            data: Some(data),
            total: Some(total),
            status: StatusCode::Success,
            msg: "success".to_string(),
        }
    }

    pub fn fail(err: BizError) -> ListResponse<T> {
        ListResponse {
            data: None,
            total: None,
            status: err.code,
            msg: err.msg,
        }
    }
}
