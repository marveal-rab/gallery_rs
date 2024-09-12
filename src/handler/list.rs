use std::cmp::PartialEq;
use actix_web::web::to;
use crate::{
    base::response::list_resp::{self, ListResponse},
    model::{dto::ObjectDTO, request::ListObjectRequest, dto::ObjectKind},
};
use crate::base::{BizError, StatusCode};
use crate::infrastructure::minio::get_minio_client;


pub async fn handle(req: ListObjectRequest) -> ListResponse<ObjectDTO> {
    if req.dir.is_none() {
        return ListResponse::fail(BizError::new_with_msg(StatusCode::Fail, "dir is required"))
    }
    return match get_minio_client().list_objects(req.dir.as_ref().unwrap()).await {
        Ok(items) => {
            let mut data: Vec<ObjectDTO> = vec![];
            let mut total = 0;
            for item in items {
                let mut url = "".to_string();
                let dir = req.dir.as_ref().unwrap().to_string();
                let name = item.name.as_str().to_string();
                let kind = match item.user_metadata {
                    Some(meta) => {
                        if meta.contains_key("content-type") {
                            ObjectKind::from_str(meta.get("content-type").unwrap())
                        } else {
                            ObjectKind::DIR
                        }
                    },
                    None => ObjectKind::UNKNOWN,
                };
                if kind == ObjectKind::IMAGE {
                    url = get_minio_client().preview(item.name.as_str()).await.unwrap_or_else(|e| {
                        println!("[list] preview err: {:?}", e);
                        "".to_string()
                    });
                }
                data.push(ObjectDTO { dir, name, url, kind });
                total += 1;
            }
            ListResponse::success(data, total)
        },
        Err(err) => {
            println!("[list] list object err: {:?}", err);
            ListResponse::success(vec![], 0)
        },
    };
}
