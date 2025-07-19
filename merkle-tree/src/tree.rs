use sha2::Digest;

use crate::hasher::Hasher;
#[derive(Clone)]
pub struct Tree<H: Hasher>
where
    H::Output: Clone + Digest,
{
    nodes: Vec<H::Output>,
    leaves: Vec<H::Output>,
    height: i32,
}

impl<H: Clone + Hasher> Tree<H>
where
    H::Output: Clone + Digest,
{
    // fn default() -> Self {
    //     Self { nodes: (), leaves: (), height: () }
    // }

    pub fn new(leaves: &[H::Output]) -> Tree<H> {
        if leaves.is_empty() {
            println!("You need to enter leaves for the merkle tree to exist")
        }
        let height = calculate_height(leaves.len());
        return Tree {
            nodes: vec![],
            leaves: leaves.to_vec(),
            height: height,
        };
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
            height + 1
        }
    }
}
