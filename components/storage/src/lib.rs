use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::primitives::ByteStream;
use std::path::Path;

pub struct StorageClient {
    pub client: Client,
    pub traces_bucket: String,
    pub proofs_bucket: String,
    pub cdn_domain: String,
}

impl StorageClient {
    pub async fn new(traces_bucket: &str, proofs_bucket: &str, cdn_domain: &str) -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Self {
            client,
            traces_bucket: traces_bucket.to_owned(),
            proofs_bucket: proofs_bucket.to_owned(),
            cdn_domain: cdn_domain.to_owned(),
        }
    }

    /// Upload an execution trace to S3
    pub async fn upload_trace(&self, trace_id: &str, file_path: &Path) -> Result<(), Error> {
        let body = ByteStream::from_path(file_path).await;
        let stream = match body {
            Ok(s) => s,
            Err(e) => panic!("Error reading file: {:?}", e),
        };

        self.client
            .put_object()
            .bucket(&self.traces_bucket)
            .key(format!("traces/{}.json", trace_id))
            .body(stream)
            .send()
            .await?;

        Ok(())
    }

    /// Download an execution trace from S3 to a local file
    pub async fn download_trace(&self, trace_id: &str, dest_path: &Path) -> Result<(), Error> {
        let mut response = self.client
            .get_object()
            .bucket(&self.traces_bucket)
            .key(format!("traces/{}.json", trace_id))
            .send()
            .await?;

        let data = response.body.collect().await.unwrap().into_bytes();
        std::fs::write(dest_path, data).expect("Failed to write trace file");

        Ok(())
    }

    /// Upload a finalized STARK proof to S3
    pub async fn upload_proof(&self, proof_id: &str, file_path: &Path) -> Result<String, Error> {
        let body = ByteStream::from_path(file_path).await;
        let stream = match body {
            Ok(s) => s,
            Err(e) => panic!("Error reading file: {:?}", e),
        };

        let key = format!("proofs/{}.bin", proof_id);

        self.client
            .put_object()
            .bucket(&self.proofs_bucket)
            .key(&key)
            .content_type("application/octet-stream")
            .send()
            .await?;

        // Return the CDN URL for fast serving
        Ok(format!("https://{}/{}", self.cdn_domain, key))
    }

    /// Get the CDN URL for a proof without downloading
    pub fn get_proof_cdn_url(&self, proof_id: &str) -> String {
        format!("https://{}/proofs/{}.bin", self.cdn_domain, proof_id)
    }
}
