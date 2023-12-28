pub trait EdgeTrait: Copy + Clone {
    fn to(&self) -> usize;
    fn rev(&self, from: usize) -> Self;
}

impl EdgeTrait for usize {
    fn to(&self) -> usize {
        *self
    }

    fn rev(&self, from: usize) -> Self {
        from
    }
}
