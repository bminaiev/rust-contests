use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;
use crate::misc::rec_function::{Callable2, RecursiveFunction2};

pub fn calc_edge_to_parent<Graph, Edge>(tree: &Graph, root_edge: Edge) -> Vec<Edge>
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
{
    let n = tree.num_vertices();
    let mut res = vec![root_edge; n];
    RecursiveFunction2::new(|f, v, parent| {
        for edge in tree.adj(v) {
            if edge.to() == parent {
                continue;
            }
            f.call(edge.to(), v);
            res[edge.to()] = edge.clone();
        }
    })
    .call(root_edge.to(), root_edge.to());
    res
}
