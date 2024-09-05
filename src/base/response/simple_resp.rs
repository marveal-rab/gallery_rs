use serde::Serialize;

#[derive(Serialize)]
pub struct SimpleResponse<T: serde::Serialize> {
    pub data: Option<T>,
    pub status: u16,
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

pub fn success<T: serde::Serialize>(data: Option<T>) -> SimpleResponse<T> {
    SimpleResponse {
        data,
        status: 0,
        msg: "success".to_string(),
    }
}
