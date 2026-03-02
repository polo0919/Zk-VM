use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tracing::{info, error};

use crate::{ProverService, execution_engine::models::ExecutionTrace};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofJob {
    pub job_id: String,
    pub trace_id: String,
    pub timestamp: u64,
}

pub struct JobWorker {
    pub consumer: StreamConsumer,
    pub redis_pool: bb8::Pool<bb8_redis::RedisConnectionManager>,
}

impl JobWorker {
    pub async fn new(brokers: &str, group_id: &str, topic: &str, redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()?;

        consumer.subscribe(&[topic])?;

        let manager = bb8_redis::RedisConnectionManager::new(redis_url)?;
        let redis_pool = bb8::Pool::builder()
            .max_size(15)
            .build(manager)
            .await?;

        Ok(Self { consumer, redis_pool })
    }

    pub async fn run(&self) {
        info!("Worker listening for proof jobs...");

        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    let payload = match msg.payload_view::<str>() {
                        None => continue,
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            error!("Error deserializing message payload: {:?}", e);
                            continue;
                        }
                    };

                    let job: ProofJob = match serde_json::from_str(payload) {
                        Ok(j) => j,
                        Err(e) => {
                            error!("Failed to parse ProofJob JSON: {:?}", e);
                            continue;
                        }
                    };

                    info!("Received job ID {} for trace {}", job.job_id, job.trace_id);
                    
                    // Mark as PROCESSING
                    if let Ok(mut conn) = self.redis_pool.get().await {
                        let _: () = conn.set_ex(format!("job_status:{}", job.job_id), "PROCESSING", 86400)
                            .await.unwrap_or_default();
                    }

                    // [SIMULATED] Fetch trace from storage (S3/Redis)
                    // In a literal implementation, we would download `trace_id` artifact.
                    let dummy_trace = ExecutionTrace {
                        steps: vec![],
                        public_inputs: vec![42],
                    };

                    let proof = ProverService::prove(dummy_trace);
                    info!("Generated proof with metadata: {:?}", proof.metadata);

                    // Mark as COMPLETED and possibly store proof somewhere
                    if let Ok(mut conn) = self.redis_pool.get().await {
                        let _: () = conn.set_ex(format!("job_status:{}", job.job_id), "COMPLETED", 86400)
                            .await.unwrap_or_default();
                    }
                }
                Err(e) => error!("Kafka error: {}", e),
            }
        }
    }
}
