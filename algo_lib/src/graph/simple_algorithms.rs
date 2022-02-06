use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

pub trait GraphAlgorithms<Edge> {
    fn any_self_loops(&self) -> bool;
    fn degree(&self, v: usize) -> usize;
}

impl<Graph, Edge> GraphAlgorithms<Edge> for Graph
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
{
    fn any_self_loops(&self) -> bool {
        for v in 0..self.len() {
            if self.adj(v).iter().any(|edge| edge.to() == v) {
                return true;
            }
        }
        false
    }

    fn degree(&self, v: usize) -> usize {
        self.adj(v).len()
    }
}
