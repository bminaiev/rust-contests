use std::ops::{Index, Range};

use crate::seg_trees::seg_tree_trait::SegTreeNode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NodeId(u32);

impl NodeId {
    pub const NONE: Self = NodeId(u32::MAX);
}

#[derive(Clone)]
pub struct TreeNode<T: SegTreeNode> {
    left: NodeId,
    right: NodeId,
    node: T,
    update: Option<T::Update>,
}

impl<T: SegTreeNode> TreeNode<T> {
    pub fn inner(&self) -> &T {
        &self.node
    }
}

pub struct PersistentSegTree<T: SegTreeNode> {
    nodes: Vec<TreeNode<T>>,
    context: T::Context,
    n: usize,
}

impl<T: SegTreeNode> Index<NodeId> for PersistentSegTree<T> {
    type Output = T;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0 as usize].node
    }
}

impl<T: SegTreeNode> PersistentSegTree<T> {
    pub fn new_f_with_context(
        n: usize,
        f: &dyn Fn(usize) -> T,
        context: T::Context,
    ) -> (Self, NodeId) {
        assert!(n > 0);
        let mut res = Self {
            nodes: vec![],
            context,
            n,
        };
        let root = res.build_f(0..n, f);
        (res, root)
    }

    pub fn build_f(&mut self, range: Range<usize>, f: &dyn Fn(usize) -> T) -> NodeId {
        let node = if range.len() == 1 {
            TreeNode {
                left: NodeId::NONE,
                right: NodeId::NONE,
                node: f(range.start),
                update: None,
            }
        } else {
            let m = (range.start + range.end) >> 1;
            let left = self.build_f(range.start..m, f);
            let right = self.build_f(m..range.end, f);
            self.unite(left, right)
        };

        self.new_node(node)
    }

    fn unite(&self, left: NodeId, right: NodeId) -> TreeNode<T> {
        TreeNode {
            left,
            right,
            node: T::join_nodes(&self.node(left).node, &self.node(right).node, &self.context),
            update: None,
        }
    }

    pub fn update(&mut self, id: NodeId, range: Range<usize>, update: &T::Update) -> NodeId {
        assert!(!range.is_empty());
        assert!(range.start < self.n);
        self.update_(self.node(id).clone(), 0..self.n, range, update)
    }

    fn update_(
        &mut self,
        node: TreeNode<T>,
        node_range: Range<usize>,
        update_range: Range<usize>,
        update: &T::Update,
    ) -> NodeId {
        if update_range.start >= node_range.end || node_range.start >= update_range.end {
            return self.new_node(node);
        }
        if update_range.start <= node_range.start && node_range.end <= update_range.end {
            let mut node: TreeNode<T> = node.clone();
            T::apply_update(&mut node.node, update);
            return self.new_node(node);
        }
        let m = node_range.start + ((node_range.end - node_range.start) >> 1);
        let mut left_id = node.left;
        let mut right_id = node.right;
        if let Some(update) = &node.update {
            let (node_left, node_right) = self.get_children(&node);
            left_id = self.update_(node_left, node_range.start..m, update_range.clone(), update);
            right_id = self.update_(node_right, m..node_range.end, update_range, update);
        } else {
            if update_range.start < m {
                left_id = self.update_(
                    self.node(left_id).clone(),
                    node_range.start..m,
                    update_range.clone(),
                    update,
                );
            }
            if update_range.end > m {
                right_id = self.update_(
                    self.node(right_id).clone(),
                    m..node_range.end,
                    update_range,
                    update,
                );
            }
        }
        let new_node = self.unite(left_id, right_id);
        self.new_node(new_node)
    }

    fn new_node(&mut self, node: TreeNode<T>) -> NodeId {
        self.nodes.push(node);
        NodeId(self.nodes.len() as u32 - 1)
    }

    pub fn get(&self, id: NodeId, range: Range<usize>) -> T {
        assert!(!range.is_empty());
        self.get_(self.node(id), 0..self.n, range)
    }

    fn get_(&self, node: &TreeNode<T>, node_range: Range<usize>, query: Range<usize>) -> T {
        assert!(!query.is_empty());
        assert!(query.end >= node_range.start);
        assert!(query.start < node_range.end);
        if query.start <= node_range.start && node_range.end <= query.end {
            return node.node.clone();
        }
        let m = node_range.start + ((node_range.end - node_range.start) >> 1);
        let (node_left, node_right) = self.get_children(node);
        if query.start >= m {
            self.get_(&node_right, m..node_range.end, query)
        } else if query.end <= m {
            self.get_(&node_left, node_range.start..m, query)
        } else {
            T::join_nodes(
                &self.get_(&node_left, node_range.start..m, query.clone()),
                &self.get_(&node_right, m..node_range.end, query),
                &self.context,
            )
        }
    }

    pub fn get_children(&self, node: &TreeNode<T>) -> (TreeNode<T>, TreeNode<T>) {
        let mut left = self.node(node.left).clone();
        let mut right = self.node(node.right).clone();
        if let Some(update) = &node.update {
            self.apply_update(&mut left, update);
            self.apply_update(&mut right, update);
        }
        (left, right)
    }

    fn apply_update(&self, node: &mut TreeNode<T>, update: &T::Update) {
        T::apply_update(&mut node.node, update);
        Self::join_updates(&mut node.update, update);
    }

    fn join_updates(current: &mut Option<T::Update>, add: &T::Update) {
        match current {
            None => *current = Some(add.clone()),
            Some(current) => T::join_updates(current, add),
        };
    }

    pub fn node(&self, id: NodeId) -> &TreeNode<T> {
        &self.nodes[id.0 as usize]
    }

    pub fn len_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }
}
