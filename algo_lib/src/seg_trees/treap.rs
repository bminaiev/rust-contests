use std::ops::Range;

use crate::{misc::rand::Random, seg_trees::seg_tree_trait::SegTreeNode};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NodeRef(u32);

impl NodeRef {
    pub const NULL: Self = Self(u32::MAX);

    pub fn is_null(&self) -> bool {
        *self == Self::NULL
    }
}

#[derive(Clone, Debug)]
struct TreapNode<T: SegTreeNode> {
    value: T,
    tree_values: T,
    // higher priority -> higher in the tree
    priority: u32,
    child: [NodeRef; 2],
    update: Option<T::Update>,
    len: u32,
}

pub struct Treap<T: SegTreeNode> {
    nodes: Vec<TreapNode<T>>,
    rng: Random,
    context: T::Context,
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
        }
    }

    pub fn new_node(&mut self, inner: T) -> NodeRef {
        let node = TreapNode {
            value: inner.clone(),
            tree_values: inner,
            priority: self.rng.gen_u64() as u32,
            child: [NodeRef::NULL, NodeRef::NULL],
            update: None,
            len: 1,
        };
        self.nodes.push(node);
        NodeRef((self.nodes.len() - 1) as u32)
    }

    fn apply_update(&mut self, node: NodeRef, update: &T::Update) {
        if node.is_null() {
            return;
        }
        T::apply_update(&mut self.get_mut(node).value, update);
        T::apply_update(&mut self.get_mut(node).tree_values, update);
        if self.get(node).len > 1 {
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
        self.get_mut(node).len =
            (self.len(self.get(node).child[0]) + self.len(self.get(node).child[1]) + 1) as u32;
        self.get_mut(node).tree_values = self.get(node).value.clone();
        if self.get(node).child[0] != NodeRef::NULL {
            self.get_mut(node).tree_values = T::join_nodes(
                &self.get(self.get(node).child[0]).tree_values,
                &self.get(node).tree_values,
                &self.context,
            );
        }
        if self.get(node).child[1] != NodeRef::NULL {
            self.get_mut(node).tree_values = T::join_nodes(
                &self.get(node).tree_values,
                &self.get(self.get(node).child[1]).tree_values,
                &self.context,
            );
        }
    }

    pub fn merge(&mut self, lhs: NodeRef, rhs: NodeRef) -> NodeRef {
        if lhs.is_null() {
            return rhs;
        }
        if rhs.is_null() {
            return lhs;
        }
        let new_node = if self.get(lhs).priority > self.get(rhs).priority {
            self.push_update(lhs);
            let new_rhs = self.merge(self.get(lhs).child[1], rhs);
            self.get_mut(lhs).child[1] = new_rhs;
            lhs
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
        assert!(pos <= self.len(root));
        if pos == 0 {
            return (NodeRef::NULL, root);
        }
        if pos == self.len(root) {
            return (root, NodeRef::NULL);
        }
        self.push_update(root);
        let left_cnt = self.len(self.get(root).child[0]);
        if pos <= left_cnt {
            let (new_left, new_right) = self.split(self.get(root).child[0], pos);
            self.get_mut(root).child[0] = new_right;
            self.recalc_node(root);
            (new_left, root)
        } else if pos == left_cnt + 1 {
            let new_right = self.get(root).child[1];
            self.get_mut(root).child[1] = NodeRef::NULL;
            self.recalc_node(root);
            (root, new_right)
        } else {
            let (new_left, new_right) = self.split(self.get(root).child[1], pos - left_cnt - 1);
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
        if !f(&self.get(root).tree_values) {
            return self.len(root);
        }
        let mut prefix: Option<T> = None;
        let mut pos = 0;
        let mut node = root;
        loop {
            self.push_update(node);
            if self.get(node).len == 1 {
                break;
            }
            let left_child = self.get(node).child[0];
            if !left_child.is_null() {
                let check_node = match &prefix {
                    Some(prefix) => {
                        T::join_nodes(prefix, &self.get(left_child).tree_values, &self.context)
                    }
                    None => self.get(left_child).tree_values.clone(),
                };
                if f(&check_node) {
                    node = self.get(node).child[0];
                    continue;
                } else {
                    pos += self.len(left_child);
                    prefix = Some(check_node);
                }
            }
            let check_node = match &prefix {
                Some(prefix) => T::join_nodes(prefix, &self.get(node).value, &self.context),
                None => self.get(node).value.clone(),
            };

            if f(&check_node) {
                break;
            } else {
                prefix = Some(check_node);
                node = self.get(node).child[1];
                pos += 1;
            }
        }
        pos
    }

    pub fn query(&mut self, root: NodeRef, range: Range<usize>) -> Option<T> {
        if root.is_null() || range.start >= range.end || range.start >= self.len(root) {
            return None;
        }
        if range.start == 0 && range.end >= self.len(root) {
            return Some(self.get(root).tree_values.clone());
        }
        self.push_update(root);
        let left_len = self.len(self.get(root).child[0]);
        if range.end <= left_len {
            return self.query(self.get(root).child[0], range);
        }
        if range.start > left_len {
            return self.query(
                self.get(root).child[1],
                range.start.saturating_sub(left_len + 1)..range.end.saturating_sub(left_len + 1),
            );
        }

        let mut res = self.get(root).value.clone();
        if let Some(left) = self.query(self.get(root).child[0], range.start..range.end) {
            res = T::join_nodes(&left, &res, &self.context);
        }
        if let Some(right) = self.query(
            self.get(root).child[1],
            range.start.saturating_sub(left_len + 1)..range.end.saturating_sub(left_len + 1),
        ) {
            res = T::join_nodes(&res, &right, &self.context);
        }
        Some(res)
    }

    pub fn insert(&mut self, root: &mut NodeRef, pos: usize, inner: T) {
        let (left, right) = self.split(*root, pos);
        let new_node = self.new_node(inner);
        let lhs = self.merge(left, new_node);
        *root = self.merge(lhs, right);
    }

    pub fn remove(&mut self, root: &mut NodeRef, pos: usize) -> &T {
        let len = self.len(*root);
        assert!(pos < len);
        let (left, right) = self.split(*root, pos);
        let (mid, right) = self.split(right, 1);
        *root = self.merge(left, right);
        &self.get(mid).tree_values
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
            stack.push(node);
            if self.get(node).len == 1 {
                break;
            }
            let left_cnt = self.len(self.get(node).child[0]);
            if pos < left_cnt {
                node = self.get(node).child[0];
            } else if pos == left_cnt {
                break;
            } else {
                pos -= left_cnt + 1;
                node = self.get(node).child[1];
            }
        }
        self.get_mut(node).value = inner;
        for node in stack.into_iter().rev() {
            self.recalc_node(node);
        }
    }

    pub fn get_node(&self, root: NodeRef) -> Option<&T> {
        if root == NodeRef::NULL {
            return None;
        }
        Some(&self.get(root).tree_values)
    }

    pub fn len(&self, root: NodeRef) -> usize {
        if root == NodeRef::NULL {
            return 0;
        }
        self.get(root).len as usize
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
}
