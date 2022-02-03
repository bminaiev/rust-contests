use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;
use crate::graph::trees::binary_lifting::BinaryLifting;
use crate::graph::trees::calc_sizes::calc_subtree_sizes;
use crate::misc::rec_function::{Callable3, RecursiveFunction3};
use std::cmp::min;
use std::ops::Range;

#[derive(Debug)]
pub struct SubPath<T> {
    pub vertices: Vec<usize>,
    pub extra: T,
}

impl<T> SubPath<T> {
    pub fn new(vertices: Vec<usize>, extra: T) -> Self {
        Self { vertices, extra }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GoDirection {
    LeftToRight,
    RightToLeft,
}

pub struct HeavyLight<T> {
    paths: Vec<SubPath<T>>,
    path_of_vertex: Vec<usize>,
    position_inside_path: Vec<usize>,
    binary_lifting: BinaryLifting,
}

impl<T> HeavyLight<T> {
    pub fn new<Graph, Edge>(
        tree: &Graph,
        root: usize,
        mut make_node: impl FnMut(&[usize]) -> T,
    ) -> Self
    where
        Graph: GraphTrait<Edge>,
        Edge: EdgeTrait,
    {
        let sizes = calc_subtree_sizes(tree, root);
        let mut paths = vec![];
        let mut path_of_vertex = vec![0; tree.num_vertices()];
        let mut position_inside_path = vec![0; tree.num_vertices()];
        RecursiveFunction3::new(|f, v, parent, mut cur_path: Vec<usize>| {
            cur_path.push(v);
            if let Some(best_child) = tree
                .adj(v)
                .iter()
                .filter(|edge| edge.to() != parent)
                .max_by_key(|edge| sizes[edge.to()])
            {
                f.call(best_child.to(), v, cur_path);
                for edge in tree.adj(v) {
                    if edge.to() == best_child.to() || edge.to() == parent {
                        continue;
                    }
                    f.call(edge.to(), v, vec![v]);
                }
            } else {
                // first node in the path is node from the next path
                for (pos, &v) in cur_path.iter().enumerate().skip(1) {
                    path_of_vertex[v] = paths.len();
                    position_inside_path[v] = pos;
                }
                let extra = make_node(&cur_path);
                paths.push(SubPath::new(cur_path, extra));
            }
        })
        .call(root, root, vec![root]);
        Self {
            paths,
            path_of_vertex,
            position_inside_path,
            binary_lifting: BinaryLifting::new(tree, root),
        }
    }

    fn construct_path(&self, mut v_from: usize, lca: usize) -> Vec<(usize, Range<usize>)> {
        let mut res = vec![];
        let mut more_len = self.binary_lifting.height(v_from) - self.binary_lifting.height(lca);
        while more_len > 0 {
            let sub_path_id = self.path_of_vertex[v_from];
            let pos_in_path = self.position_inside_path[v_from];
            let use_len = min(more_len, pos_in_path);
            let range = pos_in_path - use_len..pos_in_path;
            res.push((sub_path_id, range));
            more_len -= use_len;
            v_from = self.paths[sub_path_id].vertices[0];
        }
        res
    }

    ///
    /// Works fine for going through edges. Maybe need something else
    /// for going through vertices...
    ///
    /// range.len() -- always number of edges in the subpath
    ///
    /// subpath[0] doesn't belong to this specific subpath,
    /// it is the "next" vertex on the parent subpath
    ///
    /// edges on the path from
    /// subpath[range.start] and subpath[range.end]
    /// should be handled inside [f]
    ///
    pub fn go_path(
        &mut self,
        v_from: usize,
        v_to: usize,
        mut f: impl FnMut(&mut SubPath<T>, Range<usize>, GoDirection),
    ) {
        let lca = self.binary_lifting.lca(v_from, v_to);
        let to_lca = self.construct_path(v_from, lca);
        for (sub_path_id, range) in to_lca.into_iter() {
            f(
                &mut self.paths[sub_path_id],
                range,
                GoDirection::RightToLeft,
            );
        }
        let mut from_lca = self.construct_path(v_to, lca);
        from_lca.reverse();
        for (sub_path_id, range) in from_lca.into_iter() {
            f(
                &mut self.paths[sub_path_id],
                range,
                GoDirection::LeftToRight,
            );
        }
    }
}
