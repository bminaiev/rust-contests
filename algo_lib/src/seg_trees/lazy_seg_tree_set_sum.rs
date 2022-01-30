use crate::misc::gen_vector::gen_vec;
use crate::misc::num_traits::Number;
use crate::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
use std::fmt::{Debug, Formatter};

#[derive(Clone, Default, Copy, Debug)]
pub struct Node<T>
where
    T: Number,
{
    pub sum: T,
    pub len: i32,
}

impl<T> LazySegTreeNodeSpec for Node<T>
where
    T: Number,
{
    #[allow(unused)]
    fn unite(l: &Self, r: &Self, context: &()) -> Self {
        Self {
            len: l.len + r.len,
            sum: l.sum + r.sum,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.sum = *update * T::from_i32(node.len);
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = T;
    type Context = ();
}

pub type SegTreeSetSum = LazySegTree<Node<i64>>;

impl SegTreeSetSum {
    pub fn values(&mut self) -> Vec<i64> {
        gen_vec(self.len(), |pos| self.get(pos, pos + 1).sum)
    }
}
