//! Single Node Prover Service
//! Consumes an ExecutionTrace, applies Arithmetization, and generates a STARK proof.

pub mod consumer;
pub mod grpc_server;

use execution_engine::models::ExecutionTrace;
use primitives::Hash;
use fri::FriProof;
use core::time::Duration;

#[derive(Debug, Clone)]
pub struct StarkProof {
    pub trace_commitment: Hash,
    pub fri_proof: fri::FriProof,
    pub public_inputs: Vec<u32>,
    pub metadata: ProofMetadata,
}

#[derive(Debug, Clone)]
pub struct ProofMetadata {
    pub proof_generation_time_ms: u64,
    pub trace_len: usize,
}

pub struct ProverService;

impl ProverService {
    pub fn prove(trace: ExecutionTrace) -> StarkProof {
        let start = std::time::Instant::now();
        
        let trace_len = trace.steps.len();
        
        // 1. Trace Arithmetization
        let commitment = plonkish::generate_trace_commitment(trace_len, 10);
        
        // 2. Polynomial Commitments (FRI)
        let evals = commitment.polynomial_evals.clone();
        let fri_proof = fri::prove_low_degree(&evals);
        
        let duration = start.elapsed();
        
        StarkProof {
            trace_commitment: commitment.root,
            fri_proof,
            public_inputs: trace.public_inputs,
            metadata: ProofMetadata {
                proof_generation_time_ms: duration.as_millis() as u64,
                trace_len,
            }
        }
    }
}
