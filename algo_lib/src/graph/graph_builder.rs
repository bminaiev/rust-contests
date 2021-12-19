use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::simple_graph::SimpleGraphT;

pub struct GraphBuilder<E>
where
    E: EdgeTrait,
{
    num_vertices: usize,
    edges: Vec<(usize, E)>,
}

impl<E> GraphBuilder<E>
where
    E: EdgeTrait,
{
    pub fn new(num_vertices: usize) -> Self {
        Self {
            num_vertices,
            edges: vec![],
        }
    }

    pub fn add_vertex(&mut self) {
        self.num_vertices += 1;
    }

    pub fn add_edge(&mut self, from: usize, edge: E) {
        self.edges.push((from, edge));
    }

    pub fn build(self) -> SimpleGraphT<E> {
        SimpleGraphT::with_edges(self.num_vertices, &self.edges)
    }
}
