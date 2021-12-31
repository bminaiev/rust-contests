use std::cmp::{max, min};
use std::ops::Range;
use std::rc::Rc;

pub trait PersistentTreeNode: Sized {
    type Update: Clone;
    type Context;

    fn apply_update(node: &PersistentTree<Self>, update: &Self::Update) -> Self;
    fn join_updates(
        ctx: &Self::Context,
        old_update: &Self::Update,
        new_update: &Self::Update,
    ) -> Self::Update;

    // arghhh....
    fn need_switch_child(update: &Self::Update) -> bool;
    fn join(ctx: &Self::Context, lhs: &PersistentTree<Self>, rhs: &PersistentTree<Self>) -> Self;
}

pub struct PersistentTree<Node>
where
    Node: PersistentTreeNode,
{
    node: Node,
    size: u32,
    child: Option<[Rc<PersistentTree<Node>>; 2]>,
    update_to_push: Option<Node::Update>,
}

impl<Node> PersistentTree<Node>
where
    Node: PersistentTreeNode,
{
    pub fn join_nodes(
        ctx: &Node::Context,
        lhs: Rc<PersistentTree<Node>>,
        rhs: Rc<PersistentTree<Node>>,
    ) -> Rc<PersistentTree<Node>> {
        Rc::new(Self {
            node: Node::join(ctx, &lhs, &rhs),
            size: lhs.size + rhs.size,
            child: Some([lhs, rhs]),
            update_to_push: None,
        })
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    fn create_range(
        ctx: &Node::Context,
        range: Range<usize>,
        f: &mut impl FnMut(usize) -> Node,
    ) -> Rc<Self> {
        if range.len() == 1 {
            Rc::new(Self {
                node: f(range.start),
                size: 1,
                child: None,
                update_to_push: None,
            })
        } else {
            let half = (range.start + range.end) >> 1;
            let left = Self::create_range(ctx, range.start..half, f);
            let right = Self::create_range(ctx, half..range.end, f);
            Self::join_nodes(ctx, left, right)
        }
    }

    pub fn create(ctx: &Node::Context, n: usize, f: &mut impl FnMut(usize) -> Node) -> Rc<Self> {
        assert!(n > 0);
        Self::create_range(ctx, 0..n, f)
    }

    fn relax(
        ctx: &Node::Context,
        node: &Rc<Self>,
    ) -> (Rc<PersistentTree<Node>>, Rc<PersistentTree<Node>>) {
        let child =
            |idx: usize| -> &Rc<PersistentTree<Node>> { &node.child.as_ref().unwrap()[idx] };

        if let Some(update) = &node.update_to_push {
            let lhs = Self::apply_update_to_node(ctx, child(0), update);
            let rhs = Self::apply_update_to_node(ctx, child(1), update);
            if Node::need_switch_child(update) {
                (rhs, lhs)
            } else {
                (lhs, rhs)
            }
        } else {
            (child(0).clone(), child(1).clone())
        }
    }

    pub fn calc(ctx: &Node::Context, node: &Rc<Self>, range: Range<usize>) -> Rc<Self> {
        assert_ne!(range.len(), 0);
        assert!(range.end <= node.size());
        if range.start == 0 && range.end == node.size() {
            return node.clone();
        }
        let (child0, child1) = Self::relax(ctx, node);

        let half = child0.size();
        if range.end <= half {
            return Self::calc(ctx, &child0, range);
        }
        if range.start >= half {
            return Self::calc(ctx, &child1, range.start - half..range.end - half);
        }
        let lhs = Self::calc(ctx, &child0, range.start..half);
        let rhs = Self::calc(ctx, &child1, 0..range.end - half);
        Self::join_nodes(ctx, lhs, rhs)
    }

    pub fn get(ctx: &Node::Context, node: &Rc<Self>, pos: usize) -> Rc<Self> {
        assert!(pos < node.size());
        if node.size() == 1 {
            return node.clone();
        }
        let (child0, child1) = Self::relax(ctx, node);
        if pos < child0.size() {
            Self::get(ctx, &child0, pos)
        } else {
            Self::get(ctx, &child1, pos - child0.size())
        }
    }

    pub fn apply_update_to_node(
        ctx: &Node::Context,
        node: &Rc<Self>,
        update: &Node::Update,
    ) -> Rc<Self> {
        let update_to_push = match &node.update_to_push {
            None => Some(update.clone()),
            Some(old_update) => Some(Node::join_updates(ctx, old_update, update)),
        };
        return Rc::new(Self {
            node: Node::apply_update(node, update),
            size: node.size,
            child: node.child.clone(),
            update_to_push,
        });
    }

    #[must_use]
    pub fn update(
        ctx: &Node::Context,
        node: &Rc<Self>,
        range: Range<usize>,
        update: &Node::Update,
    ) -> Rc<Self> {
        assert_ne!(range.len(), 0);
        assert!(range.end <= node.size());
        if range.start == 0 && range.end == node.size() {
            return Self::apply_update_to_node(ctx, node, update);
        }

        let (child0, child1) = Self::relax(ctx, node);

        let half = child0.size();
        let lhs = if range.start >= half {
            child0
        } else {
            Self::update(ctx, &child0, range.start..min(range.end, half), update)
        };

        let rhs = if range.end <= half {
            child1
        } else {
            Self::update(
                ctx,
                &child1,
                max(half, range.start - half)..range.end - half,
                update,
            )
        };

        Self::join_nodes(ctx, lhs, rhs)
    }
}
