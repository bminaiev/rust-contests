use crate::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};

#[derive(Clone, Default, Copy, Debug)]
pub struct MaxValNode {
    pub max_val: i32,
    pub pos: usize,
}

impl LazySegTreeNodeSpec for MaxValNode {
    #[allow(unused)]
    fn unite(l: &Self, r: &Self, context: &()) -> Self {
        if l.max_val > r.max_val {
            *l
        } else {
            *r
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_val = *update;
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        unreachable!()
    }

    type Update = i32;
    type Context = ();
}

pub type SegTreeMax = LazySegTree<MaxValNode>;
