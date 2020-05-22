use std::str::FromStr;

use rusoto_core::{HttpClient, Region, RusotoError};
use rusoto_credential::StaticProvider;
use rusoto_s3::{
    DeleteObjectRequest, GetObjectError, GetObjectRequest, HeadObjectError, HeadObjectRequest,
    PutObjectRequest, S3Client, StreamingBody, S3,
};
use tokio::io::AsyncReadExt;

use async_trait::async_trait;
use hold::blob::Blob;
use hold::error::Error;
use hold::provider::Provider;

/// Hold Provider for S3-compatible object storage services
pub struct S3Provider {
    s3: S3Client,
    bucket: String,
}

impl S3Provider {
    pub fn new(bucket: String) -> S3Provider {
        let s3 = S3Client::new(Region::default());
        S3Provider { s3, bucket }
    }

    pub fn new_with_config(config: S3Config) -> S3Provider {
        let bucket = config.bucket;
        let region = match config.region {
            Some(region) => Region::from_str(region.as_str()).unwrap_or(Region::default()),
            None => Region::default(),
        };

        let region = match config.endpoint {
            Some(endpoint) => Region::Custom {
                name: region.name().to_string(),
                endpoint,
            },
            None => region,
        };

        let s3 = match config.credentials {
            Some(creds) => {
                let provider =
                    StaticProvider::new_minimal(creds.access_key_id, creds.secret_access_key);
                S3Client::new_with(HttpClient::new().unwrap(), provider, region)
            }
            None => S3Client::new(region),
        };

        S3Provider { bucket, s3 }
    }
}

#[async_trait]
impl Provider for S3Provider {
    async fn get_blob(&self, key: &str) -> hold::Result<Option<Blob>> {
        log::debug!("Fetching blob {}", key);
        let req = GetObjectRequest {
            bucket: self.bucket.clone(),
            key: key.to_string(),
            ..GetObjectRequest::default()
        };

        let output = match self.s3.get_object(req).await {
            Ok(output) => output,
            Err(err) => {
                return match err {
                    RusotoError::Service(err) => match err {
                        GetObjectError::NoSuchKey(_) => {
                            log::debug!("Blob {} not found", key);
                            Ok(None)
                        }
                    },
                    _ => Err(Error::provider(err)),
                };
            }
        };
        let mut buf = Vec::new();
        output
            .body
            .unwrap()
            .into_async_read()
            .read_to_end(&mut buf)
            .await
            .map_err(|err| Error::provider(err))
            .map(|_| Some(Blob::new(key.to_string(), buf)))
    }

    async fn store_blob(&self, blob: Blob) -> hold::Result<Blob> {
        log::debug!("Storing blob {} of {} bytes", blob.key(), blob.size());
        let req = PutObjectRequest {
            bucket: self.bucket.clone(),
            key: blob.key().clone(),
            body: Some(StreamingBody::from(blob.content().clone())),
            ..PutObjectRequest::default()
        };

        self.s3
            .put_object(req)
            .await
            .map(|_| blob)
            .map_err(|err| Error::provider(err))
    }

    async fn is_blob_present(&self, key: &str) -> hold::Result<bool> {
        log::debug!("Checking blob {} presence", key);
        let req = HeadObjectRequest {
            bucket: self.bucket.clone(),
            key: key.to_string(),
            ..HeadObjectRequest::default()
        };

        let res = self.s3.head_object(req).await;
        match res {
            Ok(_) => {
                log::debug!("Blob {} found", key);
                Ok(true)
            }
            Err(err) => match &err {
                RusotoError::Service(err) => match err {
                    HeadObjectError::NoSuchKey(_) => {
                        log::debug!("Blob {} not found", key);
                        Ok(false)
                    }
                },
                RusotoError::Unknown(response) => {
                    if response.status == 404 {
                        log::debug!("Blob {} not found", key);
                        Ok(false)
                    } else {
                        Err(Error::provider(err))
                    }
                }
                _ => Err(Error::provider(err)),
            },
        }
    }

    async fn delete_blob(&self, key: &str) -> hold::Result<()> {
        log::debug!("Deleting blob {}", key);
        let req = DeleteObjectRequest {
            bucket: self.bucket.clone(),
            key: key.to_string(),
            ..DeleteObjectRequest::default()
        };

        self.s3
            .delete_object(req)
            .await
            .map(|_| ())
            .map_err(|err| Error::provider(err))
    }
}

pub struct S3Config {
    pub bucket: String,
    pub endpoint: Option<String>,
    pub region: Option<String>,
    pub credentials: Option<S3Credentials>,
}

pub struct S3Credentials {
    pub access_key_id: String,
    pub secret_access_key: String,
}
