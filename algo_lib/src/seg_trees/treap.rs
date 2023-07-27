use std::ops::Range;

use crate::{misc::rand::Random, seg_trees::lazy_seg_tree::SegTreeNode};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NodeRef(u32);

impl NodeRef {
    pub const NULL: Self = Self(u32::MAX);

    pub fn is_null(&self) -> bool {
        *self == Self::NULL
    }
}

#[derive(Clone)]
struct TreapNode<T: SegTreeNode> {
    inner: T,
    // higher priority -> higher in the tree
    priority: u32,
    // Invariant: either both children are null or none of them is null
    child: [NodeRef; 2],
    update: Option<T::Update>,
    cnt_leafs: u32,
}

impl<T: SegTreeNode> TreapNode<T> {
    pub fn cnt_leafs(&self) -> usize {
        self.cnt_leafs as usize
    }
}

pub struct Treap<T: SegTreeNode> {
    nodes: Vec<TreapNode<T>>,
    rng: Random,
    context: T::Context,
    free_ids: Vec<NodeRef>,
}

impl<T: SegTreeNode> Treap<T> {
    pub fn new() -> Self
    where
        T::Context: Default,
    {
        Self {
            nodes: Vec::new(),
            rng: Random::new(787788),
            context: T::Context::default(),
            free_ids: Vec::new(),
        }
    }

    pub fn new_node(&mut self, inner: T) -> NodeRef {
        let node = TreapNode {
            inner,
            priority: self.rng.gen_u64() as u32,
            child: [NodeRef::NULL, NodeRef::NULL],
            update: None,
            cnt_leafs: 1,
        };
        if let Some(id) = self.free_ids.pop() {
            self.nodes[id.0 as usize] = node;
            id
        } else {
            self.nodes.push(node);
            NodeRef((self.nodes.len() - 1) as u32)
        }
    }

    fn new_inner_node(&mut self, lhs: NodeRef, rhs: NodeRef) -> NodeRef {
        let node = TreapNode {
            inner: T::default(),
            priority: self.get(lhs).priority.max(self.get(rhs).priority),
            child: [lhs, rhs],
            update: None,
            cnt_leafs: self.get(lhs).cnt_leafs + self.get(rhs).cnt_leafs,
        };
        if let Some(id) = self.free_ids.pop() {
            self.nodes[id.0 as usize] = node;
            id
        } else {
            self.nodes.push(node);
            NodeRef((self.nodes.len() - 1) as u32)
        }
    }

    fn apply_update(&mut self, node: NodeRef, update: &T::Update) {
        if node.is_null() {
            return;
        }
        T::apply_update(&mut self.get_mut(node).inner, update);
        if self.get(node).cnt_leafs > 1 {
            match &mut self.get_mut(node).update {
                Some(existing_update) => T::join_updates(existing_update, update),
                None => self.get_mut(node).update = Some(update.clone()),
            }
        }
    }

    fn push_update(&mut self, node: NodeRef) {
        if let Some(update) = self.get(node).update.clone() {
            self.get_mut(node).update = None;
            self.apply_update(self.get(node).child[0], &update);
            self.apply_update(self.get(node).child[1], &update);
        }
    }

    fn recalc_node(&mut self, node: NodeRef) {
        assert!(self.get(node).update.is_none());
        if self.get(node).child[0] == NodeRef::NULL {
            return;
        }
        self.get_mut(node).cnt_leafs = self.get(self.get(node).child[0]).cnt_leafs
            + self.get(self.get(node).child[1]).cnt_leafs;
        self.get_mut(node).inner = T::join_nodes(
            &self.get(self.get(node).child[0]).inner,
            &self.get(self.get(node).child[1]).inner,
            &self.context,
        );
    }

    pub fn merge(&mut self, lhs: NodeRef, rhs: NodeRef) -> NodeRef {
        if lhs.is_null() {
            return rhs;
        }
        if rhs.is_null() {
            return lhs;
        }
        let new_node = if self.get(lhs).priority > self.get(rhs).priority {
            if self.get(lhs).cnt_leafs == 1 {
                self.new_inner_node(lhs, rhs)
            } else {
                self.push_update(lhs);
                let new_rhs = self.merge(self.get(lhs).child[1], rhs);
                self.get_mut(lhs).child[1] = new_rhs;
                lhs
            }
        } else if self.get(rhs).cnt_leafs == 1 {
            self.new_inner_node(lhs, rhs)
        } else {
            self.push_update(rhs);
            let new_lhs = self.merge(lhs, self.get(rhs).child[0]);
            self.get_mut(rhs).child[0] = new_lhs;
            rhs
        };
        self.recalc_node(new_node);
        new_node
    }

    // first pos elements go to the left
    pub fn split(&mut self, root: NodeRef, pos: usize) -> (NodeRef, NodeRef) {
        if root.is_null() {
            return (NodeRef::NULL, NodeRef::NULL);
        }
        assert!(pos <= self.get(root).cnt_leafs());
        if pos == 0 {
            return (NodeRef::NULL, root);
        }
        if pos == self.len(root) {
            return (root, NodeRef::NULL);
        }
        self.push_update(root);
        let left_cnt = self.get(self.get(root).child[0]).cnt_leafs();
        if pos <= left_cnt {
            let (new_left, new_right) = self.split(self.get(root).child[0], pos);
            if new_right.is_null() {
                self.free_ids.push(root);
                return (new_left, self.get(root).child[1]);
            }
            self.get_mut(root).child[0] = new_right;
            self.recalc_node(root);
            (new_left, root)
        } else {
            let (new_left, new_right) = self.split(self.get(root).child[1], pos - left_cnt);
            if new_left.is_null() {
                self.free_ids.push(root);
                return (self.get(root).child[0], new_right);
            }
            self.get_mut(root).child[1] = new_left;
            self.recalc_node(root);
            (root, new_right)
        }
    }

    // first pos such that f(query(0..=pos)) = true
    // if no such pos exists, returns the number of leafs
    pub fn find_first_true_pos(&mut self, root: NodeRef, mut f: impl FnMut(&T) -> bool) -> usize {
        if root.is_null() {
            return 0;
        }
        if !f(&self.get(root).inner) {
            return self.get(root).cnt_leafs();
        }
        let mut prefix: Option<T> = None;
        let mut pos = 0;
        let mut node = root;
        loop {
            self.push_update(node);
            if self.get(node).cnt_leafs == 1 {
                break;
            }
            let check_node = match &prefix {
                Some(prefix) => T::join_nodes(
                    prefix,
                    &self.get(self.get(node).child[0]).inner,
                    &self.context,
                ),
                None => self.get(self.get(node).child[0]).inner.clone(),
            };
            if f(&check_node) {
                node = self.get(node).child[0];
            } else {
                let left_cnt = self.get(self.get(node).child[0]).cnt_leafs();
                prefix = Some(check_node);
                node = self.get(node).child[1];
                pos += left_cnt;
            }
        }
        pos
    }

    pub fn query(&mut self, root: NodeRef, range: Range<usize>) -> Option<T> {
        if root.is_null() || range.start >= range.end || range.start >= self.get(root).cnt_leafs() {
            return None;
        }
        if range.start == 0 && range.end >= self.get(root).cnt_leafs() {
            return Some(self.get(root).inner.clone());
        }
        self.push_update(root);
        let left_part = self.query(self.get(root).child[0], range.start..range.end);
        let left_cnt = self.get(self.get(root).child[0]).cnt_leafs();
        let right_part = self.query(
            self.get(root).child[1],
            range.start.saturating_sub(left_cnt)..range.end.saturating_sub(left_cnt),
        );
        if left_part.is_none() {
            return right_part;
        }
        if right_part.is_none() {
            return left_part;
        }
        Some(T::join_nodes(
            &left_part.unwrap(),
            &right_part.unwrap(),
            &self.context,
        ))
    }

    pub fn insert(&mut self, root: &mut NodeRef, pos: usize, inner: T)
    where
        T: std::fmt::Debug,
    {
        let (left, right) = self.split(*root, pos);
        let new_node = self.new_node(inner);
        let lhs = self.merge(left, new_node);
        *root = self.merge(lhs, right);
    }

    pub fn remove(&mut self, root: &mut NodeRef, pos: usize) -> &T {
        let cnt_leafs = self.len(*root);
        assert!(pos < cnt_leafs);
        let (left, right) = self.split(*root, pos);
        let (mid, right) = self.split(right, 1);
        *root = self.merge(left, right);
        &self.get(mid).inner
    }

    pub fn update(&mut self, root: &mut NodeRef, range: Range<usize>, update: &T::Update) {
        if range.is_empty() {
            return;
        }
        let (lhs, rhs) = self.split(*root, range.start);
        let (mid, rhs) = self.split(rhs, range.end - range.start);
        self.apply_update(mid, update);
        let rhs = self.merge(mid, rhs);
        *root = self.merge(lhs, rhs);
    }

    pub fn update_point(&mut self, root: NodeRef, mut pos: usize, inner: T) {
        assert!(self.len(root) > pos);
        let mut stack = vec![];
        let mut node = root;
        loop {
            if self.get(node).cnt_leafs == 1 {
                break;
            }
            stack.push(node);
            let left_cnt = self.get(self.get(node).child[0]).cnt_leafs();
            if pos < left_cnt {
                node = self.get(node).child[0];
            } else {
                pos -= left_cnt;
                node = self.get(node).child[1];
            }
        }
        self.get_mut(node).inner = inner;
        for node in stack.into_iter().rev() {
            self.recalc_node(node);
        }
    }

    pub fn get_node(&self, root: NodeRef) -> Option<&T> {
        if root == NodeRef::NULL {
            return None;
        }
        Some(&self.get(root).inner)
    }

    pub fn len(&self, root: NodeRef) -> usize {
        if root == NodeRef::NULL {
            return 0;
        }
        self.get(root).cnt_leafs()
    }

    fn get(&self, node: NodeRef) -> &TreapNode<T> {
        assert_ne!(node, NodeRef::NULL);
        &self.nodes[node.0 as usize]
    }

    fn get_mut(&mut self, node: NodeRef) -> &mut TreapNode<T> {
        &mut self.nodes[node.0 as usize]
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn gc(&mut self, mid: NodeRef) {
        self.free_ids.push(mid);
    }
}
