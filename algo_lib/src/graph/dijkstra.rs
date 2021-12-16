use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::graph::GraphT;
use crate::misc::num_traits::Number;
use std::collections::BTreeSet;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Vertex<T>
where
    T: Number,
    T: Ord,
{
    pub dist: T,
    v: usize,
}

pub fn dijkstra<T>(graph: &GraphT<WeightedEdge<T>>, source: usize) -> Vec<Vertex<T>>
where
    T: Number,
    T: Ord,
{
    let n = graph.vertices_num();
    let mut vertices: Vec<_> = (0..n).map(|v| Vertex { dist: T::MAX, v }).collect();
    let mut was = vec![false; n];

    vertices[source] = Vertex {
        dist: T::ZERO,
        v: source,
    };

    let mut heap = BTreeSet::new();
    heap.insert(vertices[source].clone());

    while !heap.is_empty() {
        let vertex = heap.iter().next().unwrap().clone();
        heap.remove(&vertex);
        if was[vertex.v] {
            continue;
        }
        was[vertex.v] = true;
        for e in graph[vertex.v].iter() {
            let new_dist = vertices[vertex.v].dist + e.cost;
            if vertices[e.to()].dist > new_dist {
                assert!(!was[e.to()]);
                vertices[e.to()] = Vertex {
                    v: e.to(),
                    dist: new_dist,
                };
                heap.insert(vertices[e.to()].clone());
            }
        }
    }
    vertices
}
