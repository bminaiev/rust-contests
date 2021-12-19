use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::edges::edge_trait::EdgeTrait;

pub struct GraphBuilder<E>
where
    E: EdgeTrait,
{
    num_vertices: usize,
    edges: Vec<(u32, E)>,
}

impl<E> GraphBuilder<E>
where
    E: EdgeTrait,
    E: Default,
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
        self.edges.push((from as u32, edge));
    }

    pub fn build(self) -> CompressedGraph<E> {
        CompressedGraph::with_edge_iter(
            self.num_vertices,
            self.edges.iter().map(|(fr, edge)| (*fr as usize, *edge)),
        )
    }
}
