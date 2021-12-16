pub trait EdgeTrait: Copy + Clone {
    fn to(&self) -> usize;
}
