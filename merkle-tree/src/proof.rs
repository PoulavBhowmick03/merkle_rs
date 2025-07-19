pub struct Proof<H> {
    root: H,
    leaf: H,
}

impl<H> Proof<H> {
    pub fn verify(&self, root: H, leaf: H) -> bool {
        true
    }
}
