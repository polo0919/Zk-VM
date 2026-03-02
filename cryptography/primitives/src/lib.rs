//! Cryptographic Primitives
//! 
//! Exposes mocked hashing (Poseidon/Blake3) and Merkle Tree constructs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(data: [u8; 32]) -> Self {
        Self(data)
    }
    
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

pub struct Poseidon;

impl Poseidon {
    pub fn hash(inputs: &[u32]) -> Hash {
        // Mock Poseidon hash: just a simple permutation over the inputs
        let mut out = [0u8; 32];
        for (i, val) in inputs.iter().enumerate() {
            out[i % 32] = out[i % 32].wrapping_add(*val as u8);
        }
        Hash(out)
    }
}

pub struct Blake3;

impl Blake3 {
    pub fn hash(inputs: &[u8]) -> Hash {
        // Mock Blake3 hash for fast out-of-circuit hashing
        let mut out = [0u8; 32];
        for (i, val) in inputs.iter().enumerate() {
            out[i % 32] ^= *val;
        }
        Hash(out)
    }
}

/// A Mocked Merkle Tree for state commitments
pub struct MerkleTree {
    pub root: Hash,
}

impl MerkleTree {
    pub fn build(leaves: &[Hash]) -> Self {
        // Mock Merkle root generation
        let mut root = [0u8; 32];
        for leaf in leaves {
            for i in 0..32 {
                root[i] ^= leaf.as_bytes()[i];
            }
        }
        Self { root: Hash(root) }
    }
}
