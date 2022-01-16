use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::simple_graph::SimpleGraphT;

pub type WeightedGraph<T> = SimpleGraphT<WeightedEdge<T>>;
