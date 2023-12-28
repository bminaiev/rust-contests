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

impl GraphTrait<usize> for [Vec<usize>] {
    fn len(&self) -> usize {
        self.len()
    }

    fn num_vertices(&self) -> usize {
        self.len()
    }

    fn num_edges(&self) -> usize {
        self.iter().map(|v| v.len()).sum()
    }

    fn adj(&self, v: usize) -> &[usize] {
        &self[v]
    }
}

impl GraphTrait<usize> for Vec<Vec<usize>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn num_vertices(&self) -> usize {
        self.len()
    }

    fn num_edges(&self) -> usize {
        self.iter().map(|v| v.len()).sum()
    }

    fn adj(&self, v: usize) -> &[usize] {
        &self[v]
    }
}
