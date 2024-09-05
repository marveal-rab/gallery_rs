use actix_web::{web, Responder, Result};

use crate::handler::{list_pic, pic_detail};
use crate::model::request::ListPicRequest;

pub async fn list_pic(query: web::Query<ListPicRequest>) -> Result<impl Responder> {
    Ok(web::Json(list_pic::handle(query.0).await))
}

pub async fn detail(path: web::Path<(u32,)>) -> Result<impl Responder> {
    let (id,) = path.into_inner();
    Ok(web::Json(pic_detail::handle(id).await))
}
