use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::graph_readers::config::*;
use crate::io::input::{Input, Readable};
use crate::misc::num_traits::Number;

fn read_directed_edges<T>(
    input: &mut Input,
    num_edges: usize,
    indexation: Indexation,
) -> Vec<(usize, WeightedEdge<T>)>
where
    T: Number + Readable,
{
    (0..num_edges)
        .map(|_| {
            let mut read_v = || -> usize {
                match indexation {
                    Indexation::FromZero => input.usize(),
                    Indexation::FromOne => input.usize() - 1,
                }
            };
            let fr = read_v();
            let to = read_v();
            let cost: T = input.read();
            (fr, WeightedEdge::new(to, cost))
        })
        .collect()
}

pub fn read_weighted_graph<T>(
    input: &mut Input,
    num_vertices: usize,
    num_edges: usize,
    directional: Directional,
    indexation: Indexation,
) -> CompressedGraph<WeightedEdge<T>>
where
    T: Number + Readable,
{
    let mut edges = read_directed_edges(input, num_edges, indexation);
    match directional {
        Directional::Directed => (),
        Directional::Undirected => {
            let mut rev_edges: Vec<_> = edges
                .iter()
                .map(|(fr, edge)| (edge.to(), WeightedEdge::new(*fr, edge.cost)))
                .collect();
            edges.append(&mut rev_edges);
        }
    };
    CompressedGraph::with_edge_iter(num_vertices, edges.iter().map(|(fr, edge)| (*fr, *edge)))
}
