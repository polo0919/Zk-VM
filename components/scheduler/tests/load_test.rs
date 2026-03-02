use scheduler::JobScheduler;

#[tokio::test]
async fn test_10k_proofs_load() {
    // In CI/CD, these would point to staging env
    let scheduler = JobScheduler::new(
        "localhost:9092", 
        "redis://localhost/", 
        "proof_jobs"
    ).await.unwrap();
    
    let start = std::time::Instant::now();
    
    // Submit 10k jobs to validate queue throughput
    for i in 0..10_000 {
        let trace_id = format!("trace_{}", i);
        let _ = scheduler.submit_job(&trace_id).await;
    }
    
    let duration = start.elapsed();
    println!("Submitted 10,000 proof generation jobs in {:?}", duration);
    
    // For 10k proofs/day, submission must be extremely fast to avoid bottlenecking provers
    assert!(duration.as_secs() < 60, "Load test throughput failed");
}
