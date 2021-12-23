use crate::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};

#[derive(Clone, Default, Copy, Debug)]
pub struct MaxValNode {
    max_val: i32,
    pos: usize,
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

#[test]
fn simple() {
    let n = 5;
    let mut seg_tree = LazySegTree::new_f(n, &|pos| MaxValNode { max_val: 0, pos }, ());
    seg_tree.modify(2, 3, 123);
    let res = seg_tree.get(0, 5);
    assert_eq!(res.max_val, 123);
    assert_eq!(res.pos, 2);
}
