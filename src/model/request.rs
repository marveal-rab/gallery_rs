use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListPicRequest {
    pub page: Option<u16>,
    pub size: Option<u16>,
}

#[derive(Deserialize)]
pub struct PicDetailRequest {
    pub id: Option<u32>,
}
