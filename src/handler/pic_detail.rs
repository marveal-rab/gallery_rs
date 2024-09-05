use crate::{
    base::response::simple_resp::{self, SimpleResponse},
    model::dto::PicDetailDTO,
};

pub async fn handle(id: u32) -> SimpleResponse<PicDetailDTO> {
    let detail = PicDetailDTO { id: id };
    let resp = simple_resp::success(Some(detail));
    resp
}
