use crate::hasher::Hasher;
pub struct Proof<H: Hasher> {
    pub siblings: Vec<H::Output>,
}

impl<H: Hasher> Proof<H> {
    pub fn new(siblings: Vec<H::Output>) -> Self {
        Proof { siblings }
    }

    pub fn is_empty(&self) -> bool {
        self.siblings.is_empty()
    }
    pub fn verify(&self, leaf: H::Output, root: H::Output) -> bool
    where
        H::Output: Clone + PartialEq,
    {
        let mut current_hash = leaf;
        let mut index = self.siblings.len();
        for sibling in &self.siblings {
            if index % 2 == 0 {
                current_hash = H::hash_parent(&current_hash, sibling);
            } else {
                current_hash = H::hash_parent(sibling, &current_hash);
            }
            index /= 2;
        }
        current_hash == root
    }
}
