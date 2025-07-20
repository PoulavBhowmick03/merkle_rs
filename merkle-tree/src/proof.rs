
pub struct Proof<Output> {
    pub siblings: Vec<Output>,
}

impl<Output> Proof<Output> {
    pub fn new(siblings: Vec<Output>) -> Self {
        Proof { siblings }
    }

    pub fn is_empty(&self) -> bool {
        self.siblings.is_empty()
    }
}
