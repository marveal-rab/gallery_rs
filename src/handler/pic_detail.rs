use crate::{
    base::response::simple_resp::{self, SimpleResponse},
    infrastructure::minio::get_minio_client,
    model::dto::PicDetailDTO,
};

pub async fn handle(id: u32) -> SimpleResponse<PicDetailDTO> {
    let detail = PicDetailDTO { id: id };
    // let _ = get_minio_client()
    //     .preview("dir/pexels-mateusz-mierzejewski-622168159-17484365.jpg")
    //     .await
    //     .unwrap();
    let _ = get_minio_client().list_objects("dir/").await.unwrap();
    let resp = simple_resp::success(Some(detail));
    resp
}
