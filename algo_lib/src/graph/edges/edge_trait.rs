pub trait EdgeTrait: Copy + Clone {
    fn to(&self) -> usize;
    fn rev(&self, from: usize) -> Self;
}
