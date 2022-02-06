use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

#[derive(Clone)]
pub struct CompressedGraph<E>
where
    E: EdgeTrait,
{
    num_vertices: usize,
    edges: Vec<E>,
    start_of_edges: Vec<u32>,
}

impl<E> CompressedGraph<E>
where
    E: EdgeTrait,
    E: Default,
{
    pub fn with_edge_iter<Iter>(num_vertices: usize, edge_iter: Iter) -> Self
    where
        Iter: Iterator<Item = (usize, E)> + Clone,
    {
        let mut num_of_edges: Vec<u32> = vec![0u32; num_vertices + 1];
        for (fr, _edge) in edge_iter.clone() {
            num_of_edges[fr] += 1;
        }
        let mut start_of_edges = num_of_edges;
        for i in 1..=num_vertices {
            start_of_edges[i] += start_of_edges[i - 1];
        }
        let mut edges = vec![E::default(); start_of_edges[num_vertices] as usize];
        for (fr, edge) in edge_iter {
            start_of_edges[fr] -= 1;
            edges[start_of_edges[fr] as usize] = edge;
        }
        Self {
            num_vertices,
            edges,
            start_of_edges,
        }
    }
}

impl<E> GraphTrait<E> for CompressedGraph<E>
where
    E: EdgeTrait,
{
    fn len(&self) -> usize {
        self.num_vertices
    }

    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    #[inline(always)]
    fn adj(&self, v: usize) -> &[E] {
        let from = self.start_of_edges[v] as usize;
        let to = self.start_of_edges[v + 1] as usize;
        &self.edges[from..to]
    }
}

impl<E> CompressedGraph<E>
where
    E: EdgeTrait,
{
    pub fn all_edges(&self) -> impl Iterator<Item = (usize, &E)> + '_ {
        (0..self.num_vertices()).flat_map(move |v| self.adj(v).iter().map(move |e| (v, e)))
    }
}
