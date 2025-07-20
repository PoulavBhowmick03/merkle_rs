#[derive(thiserror::Error, Debug)]
pub enum ProofError {
    #[error("cannot build a Merkle tree with zero leaves")]
    EmptyTree,
    #[error("leaf index out of bounds")]
    LeafIndexOutOfBounds,
    #[error("no leaves in the tree")]
    NoLeaves,
    #[error("no nodes in the tree")]
    NoNodes,
    #[error("tree height is not valid")]
    InvalidHeight,
    #[error("index {index} is out of bounds for a tree with {leaf_count} leaves")]
    IndexOutOfBounds { index: usize, leaf_count: usize },
}
