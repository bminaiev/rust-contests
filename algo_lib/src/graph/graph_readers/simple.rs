use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::graph_readers::config::*;
use crate::io::input::Input;

fn read_directed_edges(
    input: &mut Input,
    num_edges: usize,
    indexation: Indexation,
) -> Vec<(usize, SimpleEdge)> {
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
            (fr, SimpleEdge::new(to))
        })
        .collect()
}

pub fn read_graph(
    input: &mut Input,
    num_vertices: usize,
    num_edges: usize,
    directional: Directional,
    indexation: Indexation,
) -> CompressedGraph<SimpleEdge> {
    let mut edges = read_directed_edges(input, num_edges, indexation);
    match directional {
        Directional::Directed => (),
        Directional::Undirected => {
            let mut rev_edges: Vec<_> = edges
                .iter()
                .map(|(fr, edge)| (edge.to(), SimpleEdge::new(*fr)))
                .collect();
            edges.append(&mut rev_edges);
        }
    };
    CompressedGraph::with_edge_iter(num_vertices, edges.iter().map(|(fr, edge)| (*fr, *edge)))
}

pub fn read_undirected_graph(
    input: &mut Input,
    num_vertices: usize,
    num_edges: usize,
) -> CompressedGraph<SimpleEdge> {
    read_graph(
        input,
        num_vertices,
        num_edges,
        Directional::Undirected,
        Indexation::FromOne,
    )
}
