use crate::{
    base::response::simple_resp::{self, SimpleResponse},
    infrastructure::minio::get_minio_client,
    model::dto::{ObjectDetailDTO,ObjectKind},
};
use crate::base::{BizError, StatusCode};
use crate::model::request::ObjectDetailRequest;

pub async fn handle(req: ObjectDetailRequest) -> SimpleResponse<ObjectDetailDTO> {
    if req.full_name.is_none() {
        return SimpleResponse::fail(BizError::new_with_msg(StatusCode::Fail, "full_name is required"));
    }
    let full_name = req.full_name.unwrap();
    let response = get_minio_client().get_object(full_name.as_str()).await.unwrap();
    let content_type = match response.headers().get("content-type") {
        Some(value) => value.to_str().unwrap(),
        None => "",
    };

    let detail = ObjectDetailDTO {
        dir: "".to_string(),
        name: "".to_string(),
        full_name,
        url: response.url().to_string(),
        kind: ObjectKind::from_str(content_type),
        content_type: content_type.to_string(),
    };

    SimpleResponse::success(Some(detail))
}
