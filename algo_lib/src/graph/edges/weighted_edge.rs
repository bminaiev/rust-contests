use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::weighted_graph::WeightedGraph;
use crate::misc::num_traits::Number;

#[derive(Copy, Clone, Default)]
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
    #[inline(always)]
    fn to(&self) -> usize {
        self.to as usize
    }

    fn rev(&self, from: usize) -> Self {
        Self {
            to: from as u32,
            cost: self.cost,
        }
    }
}

impl<T: Number> WeightedGraph<T> {
    pub fn add_weighted_edge(&mut self, fr: usize, to: usize, cost: T) {
        self.add_complex_edge(fr, WeightedEdge::new(to, cost));
    }

    pub fn add_bi_weighted_edge(&mut self, fr: usize, to: usize, cost: T) {
        self.add_complex_edge(fr, WeightedEdge::new(to, cost));
        self.add_complex_edge(to, WeightedEdge::new(fr, cost));
    }
}
