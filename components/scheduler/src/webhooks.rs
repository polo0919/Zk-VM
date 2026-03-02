//! Webhook Dispatcher
//! 
//! Handles sending real-time HTTP events (`proof.ready`, `job.failed`, `verification.result`)
//! directly to enterprise client webhook endpoints.

use reqwest::Client;
use serde::Serialize;
use tracing::{info, error};

#[derive(Serialize, Debug, Clone)]
pub struct WebhookEvent {
    pub event_type: String,
    pub job_id: String,
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

pub struct WebhookDispatcher {
    client: Client,
    // Base URL or specific endpoints configurable by tenant/client ID
}

impl WebhookDispatcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }

    /// Dispatches an event payload asynchronously.
    /// In production, this would use an exponential backoff retry strategy.
    pub async fn dispatch(&self, target_url: &str, event: WebhookEvent) -> Result<(), &'static str> {
        let resp = self.client.post(target_url)
            .json(&event)
            .send()
            .await;

        match resp {
            Ok(r) if r.status().is_success() => {
                info!("Successfully dispatched {} for job {}", event.event_type, event.job_id);
                Ok(())
            }
            Ok(r) => {
                error!("Webhook returned non-200 status: {}", r.status());
                Err("Webhook HTTP Error")
            }
            Err(e) => {
                error!("Failed to reach webhook endpoint: {:?}", e);
                Err("Network Request Failed")
            }
        }
    }
}
