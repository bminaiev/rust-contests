use crate::misc::binary_search::binary_search_last_true;
use crate::misc::num_traits::Number;
use crate::seg_trees::persistent_tree_rc::{
    PersistentTree, PersistentTreeNode, PersistentTreeWithoutLinks,
};
use std::cmp::{min, Ordering};
use std::ops::Range;
use std::rc::Rc;

pub struct Context<M>
where
    M: Number,
{
    powers: Vec<M>,
    #[allow(unused)]
    multiplier: M,
}

#[derive(Copy, Clone)]
pub struct Node<M>
where
    M: Number,
{
    hash: M,
    hash_rev: M,
}

impl<M> Node<M>
where
    M: Number,
{
    pub fn new(c: u8) -> Self {
        let hash = M::from_i32(c as i32);
        Self {
            hash,
            hash_rev: hash,
        }
    }
}

impl<M> PersistentTreeNode for Node<M>
where
    M: Number,
{
    type Update = bool;
    type Context = Context<M>;

    fn apply_update(node: &PersistentTree<Self>, _update: &Self::Update) -> Self {
        Self {
            hash: node.node().hash_rev,
            hash_rev: node.node().hash,
        }
    }

    fn join_updates(
        _ctx: &Self::Context,
        old_update: &Self::Update,
        new_update: &Self::Update,
    ) -> Self::Update {
        old_update ^ new_update
    }

    fn need_switch_child(update: &Self::Update) -> bool {
        *update
    }

    fn join(
        ctx: &Self::Context,
        lhs: &PersistentTreeWithoutLinks<Self>,
        rhs: &PersistentTreeWithoutLinks<Self>,
    ) -> Self {
        Self {
            hash: lhs.node().hash * ctx.powers[rhs.size()] + rhs.node().hash,
            hash_rev: rhs.node().hash_rev * ctx.powers[lhs.size()] + lhs.node().hash_rev,
        }
    }
}

pub fn default_tree_hash_string_context<M>(max_len: usize) -> Context<M>
where
    M: Number,
{
    Context::new(max_len, M::from_i32(239017))
}

impl<M> Context<M>
where
    M: Number,
{
    pub fn new(max_len: usize, multiplier: M) -> Self {
        let mut powers = Vec::with_capacity(max_len + 1);
        powers.push(M::ONE);
        for i in 1..=max_len {
            powers.push(powers[i - 1] * multiplier);
        }
        Self { powers, multiplier }
    }

    #[must_use]
    pub fn create_from_string(&self, s: &[u8]) -> Rc<PersistentTree<Node<M>>> {
        assert_ne!(s.len(), 0);
        PersistentTree::create(self, s.len(), &mut |pos| Node::new(s[pos]))
    }

    pub fn substring(
        &self,
        str: &Rc<PersistentTree<Node<M>>>,
        range: Range<usize>,
    ) -> Rc<PersistentTree<Node<M>>> {
        PersistentTree::calc_and_save(self, str, range)
    }

    pub fn rev(&self, str: &Rc<PersistentTree<Node<M>>>) -> Rc<PersistentTree<Node<M>>> {
        PersistentTree::update(self, str, 0..str.size(), &true)
    }

    pub fn concat(
        &self,
        str1: &Rc<PersistentTree<Node<M>>>,
        str2: &Rc<PersistentTree<Node<M>>>,
    ) -> Rc<PersistentTree<Node<M>>> {
        PersistentTree::join_nodes(self, str1.clone(), str2.clone())
    }

    pub fn lcp(
        &self,
        str1: &Rc<PersistentTree<Node<M>>>,
        str2: &Rc<PersistentTree<Node<M>>>,
    ) -> usize {
        let max_possible = min(str1.size(), str2.size());
        binary_search_last_true(0..max_possible + 1, |len| {
            let node1 = PersistentTree::calc(self, str1, 0..len);
            let node2 = PersistentTree::calc(self, str2, 0..len);
            node1.node().hash == node2.node().hash
        })
        .unwrap()
    }

    pub fn get(&self, str: &Rc<PersistentTree<Node<M>>>, pos: usize) -> Option<u8> {
        if str.size() <= pos {
            return None;
        }
        let node = PersistentTree::get(self, str, pos);
        Some(node.node().hash.to_i32() as u8)
    }

    pub fn compare(
        &self,
        str1: &Rc<PersistentTree<Node<M>>>,
        str2: &Rc<PersistentTree<Node<M>>>,
    ) -> Ordering {
        let lcp = self.lcp(str1, str2);
        let c1 = self.get(str1, lcp);
        let c2 = self.get(str2, lcp);
        match (c1, c2) {
            (Some(c1), Some(c2)) => c1.cmp(&c2),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }

    pub fn to_string(&self, str: &Rc<PersistentTree<Node<M>>>) -> Vec<u8> {
        let len = str.size();
        let mut res = Vec::with_capacity(len);
        for pos in 0..len {
            res.push(self.get(str, pos).unwrap());
        }
        res
    }
}
