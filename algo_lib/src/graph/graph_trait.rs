use crate::graph::edges::edge_trait::EdgeTrait;

pub trait GraphTrait<E>
where
    E: EdgeTrait,
{
    fn num_vertices(&self) -> usize;

    fn adj(&self, v: usize) -> &[E];
}
