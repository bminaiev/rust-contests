use crate::math::modulo::Mod7;
use crate::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};

type Mod = Mod7;

#[derive(Clone, Default, Copy, Debug)]
struct HashNode {
    hash: Mod,
    len: usize,
}

struct Context {
    pow: Vec<Mod>,
}

impl LazySegTreeNodeSpec for HashNode {
    fn unite(l: &Self, r: &Self, context: &Context) -> Self {
        Self {
            hash: l.hash * context.pow[r.len] + r.hash,
            len: l.len + r.len,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.hash = Mod::new(*update as i32);
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        unreachable!()
    }

    type Update = u8;
    type Context = Context;
}

#[test]
fn simple() {
    let s = "abacaba".to_owned().into_bytes();
    let mut powers = vec![Mod::ONE; s.len() + 1];
    for i in 1..powers.len() {
        powers[i] = powers[i - 1] * Mod::new(239);
    }
    let context = Context { pow: powers };
    let mut seg_tree = LazySegTree::new_f(
        s.len(),
        &|pos| HashNode {
            hash: Mod::new(s[pos] as i32),
            len: 1,
        },
        context,
    );
    let aba = seg_tree.get(0, 3);
    let aba2 = seg_tree.get(4, 7);
    assert_eq!(aba.hash, aba2.hash);
    assert_ne!(aba2.hash, Mod7::ZERO);
}
