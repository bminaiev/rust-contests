use crate::graph::edges::edge_trait::EdgeTrait;

pub trait GraphTrait<'a, E: 'a>
where
    E: EdgeTrait,
{
    type OneNodeEdgeIter: Iterator<Item = &'a E>;
    type AllNodesEdgeIter: Iterator<Item = (usize, &'a E)>;

    fn num_vertices(&self) -> usize;
    fn all_edges(&'a self) -> Self::AllNodesEdgeIter;

    fn adj(&'a self, v: usize) -> Self::OneNodeEdgeIter;
}
