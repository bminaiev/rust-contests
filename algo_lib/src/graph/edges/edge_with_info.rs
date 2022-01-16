use crate::graph::edges::edge_trait::EdgeTrait;

#[derive(Copy, Clone, Default, Debug)]
pub struct EdgeWithInfo<T>
where
    T: Clone + Copy,
{
    to: u32,
    pub info: T,
}

impl<T> EdgeWithInfo<T>
where
    T: Clone + Copy,
{
    pub fn new(to: usize, info: T) -> Self {
        Self {
            to: to as u32,
            info,
        }
    }
}

impl<T> EdgeTrait for EdgeWithInfo<T>
where
    T: Clone + Copy,
{
    #[inline(always)]
    fn to(&self) -> usize {
        self.to as usize
    }
}
