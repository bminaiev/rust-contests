use crate::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};

#[derive(Clone, Default, Copy, Debug)]
pub struct MaxValNode<T> {
    pub max_val: T,
    pub pos: usize,
}

impl<T> LazySegTreeNodeSpec for MaxValNode<T>
where
    T: Default + Clone + Ord + Copy,
{
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
        *current = *add;
    }

    type Update = T;
    type Context = ();
}

pub type SegTreeMax<T> = LazySegTree<MaxValNode<T>>;
