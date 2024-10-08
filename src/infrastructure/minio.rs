use std::path::Path;

use actix_web::http::Method;
use chrono::Duration;
use minio::s3::args::{AbortMultipartUploadArgs, BucketExistsArgs, CompleteMultipartUploadArgs, CreateMultipartUploadArgs, GetObjectArgs, GetPresignedObjectUrlArgs, ListObjectsArgs, ListObjectsV2Args, MakeBucketArgs, PostPolicy, SelectObjectContentArgs, UploadObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::response::UploadObjectResponse;
use minio::s3::types::Item;
use minio::s3::utils::utc_now;
use reqwest::Response;
use uuid::Uuid;

use crate::config;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct MinioClient {
    client: Client,

    bucket_name: &'static str,
}

impl MinioClient {
    pub fn get<'a>() -> &'a Self {
        lazy_static::lazy_static! {
            static ref MINIO_CLIENT: MinioClient = {
                let minio_conf = config::get().minio.as_ref().expect("minio config not found");
                let access_key: &str = match &minio_conf.access_key {
                    Some(key) => key.as_str(),
                    None => panic!("access_key not found"),
                };
                let secret_access: &str = match &minio_conf.secret_key {
                    Some(key) => key.as_str(),
                    None => panic!("secret_key not found"),
                };
                let base_url = match &minio_conf.base_url {
                    Some(url) => url.parse::<BaseUrl>().unwrap(),
                    None => panic!("base_url not found"),
                };
                let bucket_name = match &minio_conf.bucket_name {
                    Some(name) => name.as_str(),
                    None => panic!("bucket_name not found"),
                };
                let static_provider = StaticProvider::new(access_key, secret_access, None);
                let client = Client::new(base_url, Some(Box::new(static_provider)), None, None).unwrap();

                MinioClient { client, bucket_name }
            };
        }
        &MINIO_CLIENT
    }

    pub async fn init_bucket(&self) -> Result<()> {
        let exists = self
            .client
            .bucket_exists(&BucketExistsArgs::new(self.bucket_name)?)
            .await?;
        if !exists {
            self.client
                .make_bucket(&MakeBucketArgs::new(self.bucket_name)?)
                .await?;
        }
        Ok(())
    }

    pub async fn upload_object(&self, full_filename: &str) -> Result<UploadObjectResponse> {
        let filename = match Path::new(full_filename).file_name() {
            Some(name) => name.to_str().ok_or_else(|| "Invalid UTF-8")?,
            None => return Err("Invalid file path".into()),
        };
        let uuid_str = Uuid::new_v4().to_string();
        let object_name = format!("{}-{}", uuid_str, filename);
        let upload_object_response = self
            .client
            .upload_object(&UploadObjectArgs::new(
                self.bucket_name,
                &object_name,
                &full_filename,
            )?)
            .await?;
        Ok(upload_object_response)
    }

    pub async fn get_object(&self, object_name: &str) -> Result<Response> {
        let get_object_response = self
            .client
            .get_object(&GetObjectArgs::new(self.bucket_name, object_name)?)
            .await?;
        Ok(get_object_response)
    }

    pub async fn preview(&self, object_name: &str) -> Result<String> {
        let get_presigned_object_url_response = self
            .client
            .get_presigned_object_url(&GetPresignedObjectUrlArgs::new(
                self.bucket_name,
                object_name,
                Method::GET,
            )?)
            .await?;
        println!("[minio] preview object: {:?}", get_presigned_object_url_response);
        Ok(get_presigned_object_url_response.url)
    }

    pub async fn list_objects(&self, dir: &str) -> Result<Vec<Item>> {
        let args = ListObjectsV2Args {
            extra_headers: None,
            extra_query_params: None,
            region: None,
            bucket: self.bucket_name,
            delimiter: Some("/"),
            encoding_type: None,
            max_keys: None,
            prefix: Some(dir),
            start_after: None,
            continuation_token: None,
            fetch_owner: false,
            include_user_metadata: true,
        };
        let list_objects_response = self
            .client
            .list_objects_v2(&args)
            .await?;
        println!("[minio] list objects: {:?}", list_objects_response);
        Ok(list_objects_response.contents)
    }

    pub async fn create_multipart_upload(&self, object_name: &str) -> Result<()> {
        let create_multipart_upload_response = self
            .client
            .create_multipart_upload(&CreateMultipartUploadArgs::new(
                self.bucket_name,
                object_name,
            )?)
            .await?;
        println!("{:?}", create_multipart_upload_response);
        Ok(())
    }

    pub async fn presign(&self, object_name: &str) -> Result<()> {
        let expiration = utc_now() + Duration::hours(1);
        let presign_response = self
            .client
            .get_presigned_post_form_data(&PostPolicy::new(self.bucket_name, &expiration)?)
            .await?;
        println!("{:?}", presign_response);
        Ok(())
    }

    pub async fn complete_multipart_upload(
        &self,
        object_name: &str,
        upload_id: &str,
    ) -> Result<()> {
        let complete_multipart_upload_response = self
            .client
            .complete_multipart_upload(&CompleteMultipartUploadArgs::new(
                self.bucket_name,
                object_name,
                upload_id,
                &Vec::new(),
            )?)
            .await?;
        println!("{:?}", complete_multipart_upload_response);
        Ok(())
    }

    pub async fn abort_multipart_upload(&self, object_name: &str, upload_id: &str) -> Result<()> {
        let abort_multipart_upload_response = self
            .client
            .abort_multipart_upload(&AbortMultipartUploadArgs::new(
                self.bucket_name,
                object_name,
                upload_id,
            )?)
            .await?;
        println!("{:?}", abort_multipart_upload_response);
        Ok(())
    }
}

pub async fn init() {
    let minio_client = MinioClient::get();
    minio_client.init_bucket().await.unwrap();
}

pub fn get_minio_client<'a>() -> &'a MinioClient {
    MinioClient::get()
}
