//! FRI (Fast Reed-Solomon Interactive Oracle Proofs of Proximity) Mock

use primitives::Hash;

#[derive(Debug, Clone)]
pub struct FriProof {
    pub root: Hash,
    pub paths: Vec<Hash>,
}

pub mod recursion;

pub fn prove_low_degree(evals: &[Hash]) -> FriProof {
    // Mock FRI Prove
    let mut sum: u8 = 0;
    for hash in evals {
        sum = sum.wrapping_add(hash.0[0]);
    }
    FriProof {
        root: Hash([sum; 32]),
        paths: evals.to_vec(),
    }
}

pub fn verify_low_degree(proof: &FriProof) -> bool {
    // Mock FRI Verify
    !proof.paths.is_empty()
}
