use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, error};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofJob {
    pub job_id: String,
    pub trace_id: String,
    pub timestamp: u64,
}

pub struct JobScheduler {
    pub producer: FutureProducer,
    pub redis_pool: bb8::Pool<bb8_redis::RedisConnectionManager>,
    pub topic: String,
}

impl JobScheduler {
    pub async fn new(kafka_brokers: &str, redis_url: &str, topic: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        let manager = bb8_redis::RedisConnectionManager::new(redis_url)?;
        let redis_pool = bb8::Pool::builder()
            .max_size(15)
            .build(manager)
            .await?;

        Ok(Self {
            producer,
            redis_pool,
            topic: topic.to_owned(),
        })
    }

    pub async fn submit_job(&self, trace_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let job_id = Uuid::new_v4().to_string();
        
        let job = ProofJob {
            job_id: job_id.clone(),
            trace_id: trace_id.to_owned(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        };

        let payload = serde_json::to_string(&job)?;

        // Update status in Redis (e.g. status: QUEUED)
        let mut conn = self.redis_pool.get().await?;
        let redis_key = format!("job_status:{}", job_id);
        let _: () = conn.set_ex(&redis_key, "QUEUED", 86400).await?;

        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&job_id);

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok((partition, offset)) => {
                info!("Job {} submitted to partition {} at offset {}", job_id, partition, offset);
                Ok(job_id)
            }
            Err((e, _)) => {
                error!("Failed to submit job to Kafka: {:?}", e);
                let _: () = conn.set_ex(&redis_key, "FAILED", 86400).await?;
                Err(Box::new(e))
            }
        }
    }

    pub async fn get_job_status(&self, job_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut conn = self.redis_pool.get().await?;
        let redis_key = format!("job_status:{}", job_id);
        let status: Option<String> = conn.get(&redis_key).await?;
        Ok(status.unwrap_or_else(|| "NOT_FOUND".to_string()))
    }
}
