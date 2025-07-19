use crate::Tree;
use sha2::{Digest, Sha256};

pub trait Hasher {
    type Output;
    fn hash_leaf(data: &[u8]) -> Self::Output;
    fn hash_parent(left: &[u8], right: &[u8]) -> Self::Output;
}

impl<H: Hasher> Tree<H>
where
    H::Output: Clone + Digest,
{
    pub fn hash_leaf(data: &[u8]) -> H::Output {
        H::hash_leaf(data)
    }

    pub fn hash_parent(left: &[u8], right: &[u8]) -> H::Output {
        H::hash_parent(left, right)
    }
}

impl<H> Hasher for Tree<H>
where
    H: Hasher,
    <H as Hasher>::Output: Clone + Digest,
{
    type Output = [u8; 32];

    fn hash_leaf(data: &[u8]) -> Self::Output {
        let hasher = Sha256::digest(data);
        let mut output = [0u8; 32];
        output.copy_from_slice(&hasher);
        output
    }
    fn hash_parent(left: &[u8], right: &[u8]) -> Self::Output {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        let mut output = [0u8; 32];
        output.copy_from_slice(&hasher.finalize());
        output
    }
}

/// A zero‑state marker for SHA‑256 hashing
pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
  type Output = [u8; 32];

  fn hash_leaf(data: &[u8]) -> Self::Output {
    let digest = Sha256::digest(data);
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&digest);
    buf
  }

  fn hash_parent(left: &[u8], right: &[u8]) -> Self::Output {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let digest = hasher.finalize();
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&digest);
    buf
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_leaf() {
        let data = b"test leaf";
        let hash = Sha256Hasher::hash_leaf(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_parent() {
        let left = b"left child";
        let right = b"right child";
        let hash = Sha256Hasher::hash_parent(left, right);
        assert_eq!(hash.len(), 32);
    }
}
