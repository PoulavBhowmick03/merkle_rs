use crate::{Proof, ProofError, hasher::Hasher};
#[derive(Clone)]
pub struct Tree<H: Hasher>
where
    H::Output: Clone,
{
    nodes: Vec<H::Output>,
    leaves: Vec<H::Output>,
    height: i32,
}

impl<H: Clone + Hasher> Tree<H>
where
    H::Output: Clone,
{
    // fn default() -> Self {
    //     Self { nodes: (), leaves: (), height: () }
    // }

    pub fn new(raw_leaves: &[&[u8]]) -> Tree<H> {
        if raw_leaves.is_empty() {
            println!("You need to enter leaves for the merkle tree to exist")
        }
        // let height = calculate_height(leaves.len());
        // return Tree {
        //     nodes: vec![],
        //     leaves: leaves.to_vec(),
        //     height: height,
        // };
        let hashed_leaves: Vec<H::Output> =
            raw_leaves.iter().map(|data| H::hash_leaf(data)).collect();
        let height = calculate_height(hashed_leaves.len());
        let mut nodes = Vec::new();

        let mut level = hashed_leaves.clone();
        while level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in level.chunks(2) {
                let left = &chunk[0];
                let right = if chunk.len() == 2 {
                    &chunk[1]
                } else {
                    &chunk[0]
                };
                // let parent = H::hash_parent(left, right);
                let parent = H::hash_parent(left, right);
                next_level.push(parent.clone());
            }
            nodes.extend_from_slice(&next_level);
            level = next_level;
        }
        Tree {
            nodes,
            leaves: hashed_leaves,
            height,
        }
    }

    pub fn root(&self) -> H::Output {
        self.nodes.last().cloned().unwrap()
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn leaves(&self) -> Vec<H::Output> {
        self.clone().leaves
    }

    pub fn prove(&self, index: usize) -> Result<Proof<H::Output>, ProofError> {
        if self.leaves.is_empty() {
            return Err(ProofError::NoLeaves);
        }
        let mut curr_index = index;

        if index >= self.leaves.len() {
            return Err(ProofError::IndexOutOfBounds {
                index,
                leaf_count: self.leaves.len(),
            });
        }

        let mut siblings = Vec::<H::Output>::new();
        let mut current = self.leaves().clone();
        let mut sib_index = 0;
        while current.len() > 1 {
            if curr_index % 2 == 0 {
                if curr_index + 1 < current.len() {
                    sib_index = curr_index + 1;
                }
                sib_index
            } else {
                sib_index = curr_index - 1;
                sib_index
            };
            siblings.push(current[sib_index].clone());
            let mut next_level = Vec::new();
            for chunk in current.chunks(2) {
                let left = &chunk[0];
                let right = if chunk.len() == 2 {
                    &chunk[1]
                } else {
                    &chunk[0]
                };
                let parent = H::hash_parent(left, right);
                next_level.push(parent);
            }
            curr_index /= 2;
            current = next_level;
        }
        Ok(Proof::new(siblings))
    }
}

pub fn calculate_height(leaves: usize) -> i32 {
    let mut height = 0;
    let mut power_of_two = 1;
    match leaves {
        0 => return 0,
        1 => return 1,
        _ => {
            while power_of_two < leaves {
                power_of_two *= 2;
                height += 1;
            }
            height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::Sha256Hasher;

    #[test]
    fn test_tree_creation() {
        let leaves: Vec<&[u8]> = vec![b"leaf1", b"leaf2", b"leaf3"];
        let tree: Tree<Sha256Hasher> = Tree::new(&leaves);
        assert_eq!(tree.leaves().len(), 3);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn test_proof() {
        let leaves: Vec<&[u8]> = vec![b"leaf1", b"leaf2", b"leaf3"];
        let tree: Tree<Sha256Hasher> = Tree::new(&leaves);
        let proof = tree.prove(1).unwrap();
        assert!(!proof.is_empty());
    }
}