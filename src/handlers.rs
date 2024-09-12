use actix_web::{web, Responder, Result};

use crate::handler::{list, detail};
use crate::model::request::{ListObjectRequest, ObjectDetailRequest};

pub async fn list(query: web::Query<ListObjectRequest>) -> Result<impl Responder> {
    Ok(web::Json(list::handle(query.0).await))
}

pub async fn detail(query: web::Query<ObjectDetailRequest>) -> Result<impl Responder> {
    Ok(web::Json(detail::handle(query.0).await))
}
