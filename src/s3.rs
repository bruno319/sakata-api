use log::*;
use rusoto_core::Region;
use rusoto_s3::{DeleteObjectRequest, PutObjectRequest, S3, S3Client};

use crate::error::SakataError;
use crate::SakataResult;
use crate::utils::http_res::server_error;

pub struct AwsS3Client {
    #[allow(dead_code)]
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl AwsS3Client {
    pub fn new() -> AwsS3Client {
        let region = Region::default();

        AwsS3Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            std::env::var("AWS_REGION").unwrap(),
            key
        )
    }

    pub async fn put_object(&self, content: Vec<u8>, key: String) -> SakataResult<String> {
        let put_request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.clone(),
            body: Some(content.into()),
            ..Default::default()
        };
        let res = self
            .s3
            .put_object(put_request)
            .await;

        match res {
            Ok(_res) => {
                info!("{} uploaded to S3", key);
                Ok(self.url(&key))
            }
            Err(e) => {
                error!("S3 Error: {}", e);
                Err(SakataError::ServerErr(server_error("Failed to upload file to S3")))
            }
        }
    }

    #[allow(dead_code)]
    pub async fn delete_object(&self, key: String) {
        let delete_object_req = DeleteObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            ..Default::default()
        };

        let _res = self
            .s3
            .delete_object(delete_object_req)
            .await
            .expect("Couldn't delete object");
    }
}