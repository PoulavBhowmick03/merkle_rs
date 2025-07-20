use crate::Tree;
use sha2::{Digest, Sha256};

pub trait Hasher {
    type Output;
    fn hash_leaf(data: &[u8]) -> Self::Output;
    fn hash_parent(left: &Self::Output, right: &Self::Output) -> Self::Output;
}

impl<H: Hasher> Tree<H>
where
    H::Output: Clone,
{
    pub fn hash_leaf(data: &[u8]) -> H::Output {
        H::hash_leaf(data)
    }

    pub fn hash_parent(left: &H::Output, right: &H::Output) -> H::Output {
        H::hash_parent(left, right)
    }
}

/// A zero‑state marker for SHA‑256 hashing
#[derive(Clone, Debug)]
pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
  type Output = [u8; 32];

  fn hash_leaf(data: &[u8]) -> Self::Output {
    let digest = Sha256::digest(data);
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    output
  }

  fn hash_parent(left: &Self::Output, right: &Self::Output) -> Self::Output {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let digest = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    output
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
        let left_hash = Sha256Hasher::hash_leaf(left);
        let right_hash = Sha256Hasher::hash_leaf(right);
        let hash = Sha256Hasher::hash_parent(&left_hash, &right_hash);
        assert_eq!(hash.len(), 32);
    }
}
