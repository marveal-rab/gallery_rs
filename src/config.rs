use std::fs::File;
use std::io::Read;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub minio: Option<MinioConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MinioConfig {
    pub base_url: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub bucket_name: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let file_path = "config.toml";
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                panic!("Error opening file {}: {}", file_path, e)
            }
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => {
                panic!("Error reading file {}: {}", file_path, e)
            }
        };
        toml::from_str(&str_val).expect("Error parsing config file")
    }
}
impl Config {
    pub fn get<'a>() -> &'a Self {
        lazy_static::lazy_static! {
            static ref CONFIG: Config = Config::default();
        }
        &CONFIG
    }
}

pub async fn init() {
    let _ = Config::get();
}

pub fn get<'a>() -> &'a Config {
    Config::get()
}
