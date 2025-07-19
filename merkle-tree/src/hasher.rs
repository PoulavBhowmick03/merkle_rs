use crate::Tree;
use sha2::{Digest, Sha256};

type Output = [u8; 32];

pub trait Hasher {
    type Output;
    fn hash_leaf(data: &[u8]) -> Output;
    fn hash_parent(left: &[u8], right: &[u8]) -> Output;
}

impl<H> Hasher for Tree<H>
where
    H: Clone + Hasher,
{
    type Output = Output;

    fn hash_leaf(data: &[u8]) -> Output {
        let hasher = Sha256::digest(data);
        let mut output = [0u8; 32];
        output.copy_from_slice(&hasher);
        output
    }
    fn hash_parent(left: &[u8], right: &[u8]) -> Output {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        let mut output = [0u8; 32];
        output.copy_from_slice(&hasher.finalize());
        output
    }
}
