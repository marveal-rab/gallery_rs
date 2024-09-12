use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListObjectRequest {
    pub dir: Option<String>,
    pub page: Option<u16>,
    pub size: Option<u16>,
}

#[derive(Deserialize)]
pub struct ObjectDetailRequest {
    pub full_name: Option<String>,
}
