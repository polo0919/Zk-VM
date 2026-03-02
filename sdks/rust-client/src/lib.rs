use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompileRequest {
    pub filename: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteRequest {
    pub program_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProveRequest {
    pub session_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyRequest {
    // Ideally this would type to the FriProof struct, but kept generic for the SDK
    pub proof: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyResponse {
    pub valid: bool,
}

pub struct ZkVMClient {
    pub base_url: String,
    pub client: Client,
}

impl ZkVMClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_owned(),
            client: Client::new(),
        }
    }

    pub async fn compile(&self, req: CompileRequest) -> Result<serde_json::Value, Error> {
        let res = self.client.post(&format!("{}/compile", self.base_url))
            .json(&req)
            .send()
            .await?;
        res.json().await
    }

    pub async fn execute(&self, req: ExecuteRequest) -> Result<serde_json::Value, Error> {
        let res = self.client.post(&format!("{}/execute", self.base_url))
            .json(&req)
            .send()
            .await?;
        res.json().await
    }

    pub async fn prove(&self, req: ProveRequest) -> Result<serde_json::Value, Error> {
        let res = self.client.post(&format!("{}/prove", self.base_url))
            .json(&req)
            .send()
            .await?;
        res.json().await
    }

    pub async fn verify(&self, req: VerifyRequest) -> Result<VerifyResponse, Error> {
        let res = self.client.post(&format!("{}/verify", self.base_url))
            .json(&req)
            .send()
            .await?;
        res.json().await
    }
}
