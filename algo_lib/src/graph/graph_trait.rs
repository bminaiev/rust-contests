use crate::graph::edges::edge_trait::EdgeTrait;

pub trait GraphTrait<E>
where
    E: EdgeTrait,
{
    // alias for [num_vertices]
    fn len(&self) -> usize;
    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;

    fn adj(&self, v: usize) -> &[E];
}
