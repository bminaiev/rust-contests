use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::dsu::Dsu;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::graph_builder::GraphBuilder;
use crate::graph::graph_trait::GraphTrait;
use crate::misc::num_traits::Number;

pub fn minimal_spanning_tree<G, T: Number>(graph: &G) -> CompressedGraph<WeightedEdge<T>>
where
    G: GraphTrait<WeightedEdge<T>>,
{
    let n = graph.num_vertices();
    let mut all_edges = Vec::with_capacity(graph.num_edges() / 2);
    for v in 0..n {
        for edge in graph.adj(v) {
            if edge.to() > v {
                all_edges.push((v, edge.clone()));
            }
        }
    }
    all_edges.sort_by_key(|(_fr, edge)| edge.cost);
    let mut dsu = Dsu::new(n);
    let mut builder = GraphBuilder::new(n);
    for (fr, edge) in all_edges.into_iter() {
        if dsu.get(fr) != dsu.get(edge.to()) {
            dsu.unite(fr, edge.to());
            builder.add_edge(fr, edge);
        }
    }
    builder.build()
}
