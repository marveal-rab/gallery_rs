use crate::{
    base::response::list_resp::{self, ListResponse},
    model::{dto::PicDTO, request::ListPicRequest},
};

pub async fn handle(req: ListPicRequest) -> ListResponse<PicDTO> {
    let pic = PicDTO {
        url: "https://www.baidu.com".to_string(),
    };
    list_resp::success(vec![pic], 100)
}
