//! PLONKish Arithmetization Mock
//! 
//! Takes an execution trace and converts it into a trace table of polynomials.

pub mod halo2_circuit;

use primitives::Hash;

pub struct ArithmetizationConfig {
    pub trace_length: usize,
    pub num_columns: usize,
}

pub struct TraceCommitment {
    pub root: Hash,
    pub polynomial_evals: Vec<u32>,
}

impl TraceCommitment {
    pub fn new(root: Hash, evals: Vec<u32>) -> Self {
        Self { root, polynomial_evals: evals }
    }
}

pub fn generate_trace_commitment(trace_length: usize, _columns: usize) -> TraceCommitment {
    // Generate a mock trace commitment
    let root = Hash::new([1u8; 32]);
    let evals = vec![0; trace_length];
    TraceCommitment::new(root, evals)
}
