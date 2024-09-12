use serde::Serialize;

#[derive(Serialize)]
pub struct ObjectDetailDTO {
    pub dir: String,
    pub name: String,
    pub full_name: String,
    pub url: String,
    pub kind: ObjectKind,
    pub content_type: String,
}

#[derive(Serialize, PartialEq)]
pub enum ObjectKind {
    UNKNOWN,
    DIR,
    IMAGE,
}

impl ObjectKind {
    pub fn from_str(s: &str) -> ObjectKind {
        match s {
            "image/jpeg" => ObjectKind::IMAGE,
            _ => ObjectKind::DIR,
        }
    }
}


#[derive(Serialize)]
pub struct ObjectDTO {
    pub dir: String,
    pub name: String,
    pub url: String,
    pub kind: ObjectKind,
}