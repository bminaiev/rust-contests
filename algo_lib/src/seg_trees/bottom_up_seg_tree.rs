use std::ops::Range;

use crate::seg_trees::seg_tree_trait::SegTreeNode;

pub struct BottomUpSegTree<Node: SegTreeNode> {
    n: usize,
    nodes: Vec<Node>,
    context: Node::Context,
}

impl<Node: SegTreeNode> BottomUpSegTree<Node> {
    pub fn new(start_n: usize, f: impl Fn(usize) -> Node) -> Self
    where
        Node::Context: Default,
    {
        let n = start_n.next_power_of_two();
        let mut res = Self {
            n,
            nodes: vec![Node::default(); 2 * n],
            context: Default::default(),
        };
        for i in 0..start_n {
            res.nodes[n + i] = f(i);
        }
        for i in (1..n).rev() {
            res.nodes[i] = Node::join_nodes(&res.nodes[2 * i], &res.nodes[2 * i + 1], &res.context);
        }
        res
    }

    pub fn update_point(&mut self, pos: usize, v: Node) {
        let mut i = pos + self.n;
        self.nodes[i] = v;
        while i > 1 {
            i /= 2;
            self.nodes[i] =
                Node::join_nodes(&self.nodes[2 * i], &self.nodes[2 * i + 1], &self.context);
        }
    }

    pub fn get_root(&self) -> &Node {
        &self.nodes[1]
    }

    pub fn get(&self, range: Range<usize>) -> Node {
        let mut l = range.start + self.n;
        let mut r = range.end + self.n;
        let mut res_l = Node::default();
        let mut res_r = Node::default();
        while l < r {
            if l & 1 != 0 {
                res_l = Node::join_nodes(&res_l, &self.nodes[l], &self.context);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                res_r = Node::join_nodes(&self.nodes[r], &res_r, &self.context);
            }
            l /= 2;
            r /= 2;
        }
        Node::join_nodes(&res_l, &res_r, &self.context)
    }
}
