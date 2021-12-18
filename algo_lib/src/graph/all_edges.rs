use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

pub fn all_graph_edges<'a, G, E: 'a>(graph: &'a G) -> impl Iterator<Item = (usize, &'a E)>
where
    G: GraphTrait<'a, E>,
    E: EdgeTrait,
{
    (0..graph.num_vertices()).flat_map(|v| graph.adj(v).map(move |e| (v, e)))
}
