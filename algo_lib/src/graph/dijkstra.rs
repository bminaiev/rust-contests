use crate::collections::min_priority_queue::MinPriorityQueue;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::graph_trait::GraphTrait;
use crate::misc::num_traits::Number;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Vertex<T>
where
    T: Number,
    T: Ord,
{
    pub dist: T,
    pub prev: usize,
    v: usize,
}

pub fn dijkstra<Graph, T>(graph: &Graph, source: usize) -> Vec<Vertex<T>>
where
    T: Number,
    T: Ord,
    Graph: GraphTrait<WeightedEdge<T>>,
{
    let n = graph.num_vertices();
    let mut vertices: Vec<_> = (0..n)
        .map(|v| Vertex {
            dist: T::MAX,
            v,
            prev: v,
        })
        .collect();

    vertices[source] = Vertex {
        dist: T::ZERO,
        v: source,
        prev: source,
    };

    let mut heap = MinPriorityQueue::new();
    heap.push(vertices[source]);

    while let Some(vertex) = heap.pop() {
        if vertices[vertex.v] != vertex {
            continue;
        }
        for e in graph.adj(vertex.v) {
            assert!(e.cost >= T::ZERO, "Negative edge");
            let new_dist = vertices[vertex.v].dist + e.cost;
            if vertices[e.to()].dist > new_dist {
                vertices[e.to()] = Vertex {
                    v: e.to(),
                    dist: new_dist,
                    prev: vertex.v,
                };
                heap.push(vertices[e.to()]);
            }
        }
    }
    vertices
}
