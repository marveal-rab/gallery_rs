use serde::Serialize;

#[derive(Serialize)]
pub struct PicDTO {
    pub url: String,
}

#[derive(Serialize)]
pub struct PicDetailDTO {
    pub id: u32,
}
