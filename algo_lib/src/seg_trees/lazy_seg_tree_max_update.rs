use std::cmp::max;

use crate::seg_trees::{lazy_seg_tree::SegTree, seg_tree_trait::SegTreeNode};

#[derive(Clone, Default, Copy, Debug)]
pub struct Node {
    pub max_val: usize,
}

impl SegTreeNode for Node {
    #[allow(unused)]
    fn join_nodes(l: &Self, r: &Self, context: &()) -> Self {
        if l.max_val > r.max_val {
            *l
        } else {
            *r
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_val = max(node.max_val, *update);
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = max(*current, *add);
    }

    type Update = usize;
    type Context = ();
}

pub type SegTreeMaxUpdate = SegTree<Node>;
