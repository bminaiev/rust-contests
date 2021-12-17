use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::simple_graph::SimpleGraphT;
use crate::io::input::{Input, Readable};
use crate::misc::num_traits::Number;

pub enum Directional {
    Directed,
    Undirected,
}

pub enum Indexation {
    FromZero,
    FromOne,
}

impl<T> SimpleGraphT<WeightedEdge<T>>
where
    T: Number,
    T: Readable,
{
    fn read_directed_edges(
        input: &mut Input,
        num_edges: usize,
        indexation: Indexation,
    ) -> Vec<(usize, WeightedEdge<T>)> {
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

    pub fn read(
        input: &mut Input,
        num_vertices: usize,
        num_edges: usize,
        directional: Directional,
        indexation: Indexation,
    ) -> Self {
        let mut edges = Self::read_directed_edges(input, num_edges, indexation);
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
        Self::with_edges(num_vertices, &edges)
    }
}
