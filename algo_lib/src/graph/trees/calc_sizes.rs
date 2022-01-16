use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;
use crate::misc::rec_function::{Callable2, RecursiveFunction2};

pub fn calc_subtree_sizes<Graph, Edge>(tree: &Graph, root: usize) -> Vec<usize>
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
{
    let n = tree.num_vertices();
    let mut res = vec![1; n];
    RecursiveFunction2::new(|f, v, parent| {
        for edge in tree.adj(v) {
            if edge.to() == parent {
                continue;
            }
            f.call(edge.to(), v);
            res[v] += res[edge.to()];
        }
    })
    .call(root, root);
    res
}
