#[derive(thiserror::Error, Debug)]
pub enum ProofError {
    #[error("cannot build a Merkle tree with zero leaves")]
    EmptyTree,
    //...
}
