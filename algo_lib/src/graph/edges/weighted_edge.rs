use crate::graph::edges::edge_trait::EdgeTrait;
use crate::misc::num_traits::Number;

#[derive(Copy, Clone)]
pub struct WeightedEdge<T>
where
    T: Number,
{
    to: u32,
    pub cost: T,
}

impl<T> WeightedEdge<T>
where
    T: Number,
{
    pub fn new(to: usize, cost: T) -> Self {
        Self {
            to: to as u32,
            cost,
        }
    }
}

impl<T> EdgeTrait for WeightedEdge<T>
where
    T: Number,
{
    fn to(&self) -> usize {
        self.to as usize
    }
}
