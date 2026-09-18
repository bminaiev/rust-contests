use crate::{
    misc::num_traits::Number,
    seg_trees::{optimized_seg_tree::OptimizedSegTree, seg_tree_trait::SegTreeNode},
};

#[derive(Clone, Default, Copy, Debug)]
pub struct Node<T: Number> {
    pub max_val: T,
}

impl<T: Number> SegTreeNode for Node<T> {
    #[allow(unused)]
    fn join_nodes(l: &Self, r: &Self, context: &()) -> Self {
        if l.max_val > r.max_val {
            *l
        } else {
            *r
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_val += *update;
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    type Update = T;
    type Context = ();
}

pub type SegTreeMaxAdd<T> = OptimizedSegTree<Node<T>>;
