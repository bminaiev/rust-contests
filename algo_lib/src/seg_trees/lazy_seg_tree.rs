use std::ops::Range;

use crate::seg_trees::seg_tree_trait::SegTreeNode;

///
/// Segment Tree
///
#[allow(unused)]
#[derive(Clone)]
pub struct SegTree<T: SegTreeNode> {
    n: usize,
    tree: Vec<T>,
    updates_to_push: Vec<Option<T::Update>>,
    context: T::Context,
    right_nodes: Vec<usize>,
}

#[allow(unused)]
impl<T: SegTreeNode> SegTree<T> {
    fn pull(&mut self, v: usize, vr: usize) {
        self.tree[v] = T::join_nodes(&self.tree[v + 1], &self.tree[vr], &self.context);
    }

    fn build(&mut self, v: usize, l: usize, r: usize, init_val: &T) {
        if l + 1 == r {
            self.tree[v] = init_val.clone();
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.build(v + 1, l, m, init_val);
            self.build(vr, m, r, init_val);
            self.pull(v, vr);
        }
    }

    fn push(&mut self, v: usize, l: usize, r: usize) {
        let update = self.updates_to_push[v].clone();
        self.updates_to_push[v] = None;
        match update {
            None => {}
            Some(update) => {
                let m = (l + r) >> 1;
                self.apply_update(v + 1, &update, m - l == 1);
                self.apply_update(v + ((r - l) & !1), &update, r - m == 1);
            }
        }
    }

    fn get_(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> T {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            return self.tree[v].clone();
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        let res = if ql >= m {
            self.get_(vr, m, r, ql, qr)
        } else if qr <= m {
            self.get_(v + 1, l, m, ql, qr)
        } else {
            T::join_nodes(
                &self.get_(v + 1, l, m, ql, qr),
                &self.get_(vr, m, r, ql, qr),
                &self.context,
            )
        };
        self.pull(v, vr);
        res
    }

    fn join_updates(current: &mut Option<T::Update>, add: &T::Update) {
        match current {
            None => *current = Some(add.clone()),
            Some(current) => T::join_updates(current, add),
        };
    }

    fn apply_update(&mut self, v: usize, update: &T::Update, is_leaf: bool) {
        T::apply_update(&mut self.tree[v], update);
        if !is_leaf {
            Self::join_updates(&mut self.updates_to_push[v], update);
        }
    }

    fn modify_(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize, update: &T::Update) {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            self.apply_update(v, update, r - l == 1);
            return;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        if ql >= m {
            self.modify_(vr, m, r, ql, qr, update);
        } else if qr <= m {
            self.modify_(v + 1, l, m, ql, qr, update);
        } else {
            self.modify_(v + 1, l, m, ql, qr, update);
            self.modify_(vr, m, r, ql, qr, update);
        };
        self.pull(v, vr);
    }

    pub fn update(&mut self, range: Range<usize>, update: T::Update) {
        if range.is_empty() {
            return;
        }
        assert!(!range.is_empty());
        self.modify_(0, 0, self.n, range.start, range.end, &update);
    }

    pub fn update_point(&mut self, pos: usize, new_node: T) {
        let mut l = 0;
        let mut r = self.n;
        let mut v: usize = 0;
        let mut to_pull = vec![];
        while r - l > 1 {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.push(v, l, r);
            to_pull.push((v, vr));
            if pos < m {
                r = m;
                v = v + 1;
            } else {
                l = m;
                v = vr;
            }
        }
        self.tree[v] = new_node;
        for (v, vr) in to_pull.into_iter().rev() {
            self.pull(v, vr);
        }
    }

    fn find_last_true_(
        &mut self,
        v: usize,
        l: usize,
        r: usize,
        range: Range<usize>,
        f: &impl Fn(&T) -> bool,
    ) -> Option<usize> {
        if range.start >= r || l >= range.end {
            return None;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        if range.start <= l && r <= range.end {
            if !f(&self.tree[v]) {
                return None;
            }
            if r - l == 1 {
                return Some(l);
            }
        }
        self.push(v, l, r);
        if let Some(res) = self.find_last_true_(vr, m, r, range.clone(), f) {
            Some(res)
        } else {
            self.find_last_true_(v + 1, l, m, range, f)
        }
    }

    // returns position
    pub fn find_last_true(&mut self, range: Range<usize>, f: impl Fn(&T) -> bool) -> Option<usize> {
        self.find_last_true_(0, 0, self.n, range, &f)
    }

    pub fn get(&mut self, range: Range<usize>) -> T {
        if range.is_empty() {
            return T::default();
        }
        self.get_(0, 0, self.n, range.start, range.end)
    }

    pub fn new_with_context(n: usize, f: impl Fn(usize) -> T, context: T::Context) -> Self {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut res = SegTree {
            n,
            tree,
            updates_to_push,
            context,
            right_nodes: vec![],
        };
        res.build_f(0, 0, n, &f);
        res
    }

    pub fn new(n: usize, f: impl Fn(usize) -> T) -> Self
    where
        T::Context: Default,
    {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut res = SegTree {
            n,
            tree,
            updates_to_push,
            context: T::Context::default(),
            right_nodes: vec![],
        };
        res.build_f(0, 0, n, &f);
        res
    }

    fn build_f(&mut self, v: usize, l: usize, r: usize, f: &impl Fn(usize) -> T) {
        if l + 1 == r {
            self.tree[v] = f(l);
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.build_f(v + 1, l, m, f);
            self.build_f(vr, m, r, f);
            self.pull(v, vr);
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn expert_get_node(&self, node: usize) -> &T {
        &self.tree[node]
    }

    pub fn expert_get_left_node(&self, node: usize) -> usize {
        node + 1
    }

    fn build_right_nodes(&mut self, v: usize, l: usize, r: usize) {
        if l + 1 == r {
            self.right_nodes.push(0);
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.right_nodes.push(vr);
            self.build_right_nodes(v + 1, l, m);
            self.build_right_nodes(vr, m, r);
        }
    }

    // TODO: shouldn't be mut
    pub fn expert_get_right_node(&mut self, node: usize) -> usize {
        if self.right_nodes.is_empty() {
            self.build_right_nodes(0, 0, self.n);
        }
        self.right_nodes[node]
    }

    // Used for Kinetic Seg Tree
    pub fn expert_rebuild_nodes(&mut self, should_rebuild: impl Fn(&T, &T::Context) -> bool) {
        self.expert_rebuild_nodes_(0, 0, self.n, &should_rebuild);
    }

    fn expert_rebuild_nodes_(
        &mut self,
        v: usize,
        l: usize,
        r: usize,
        should_rebuild: &impl Fn(&T, &T::Context) -> bool,
    ) {
        if r - l <= 1 || !should_rebuild(&self.tree[v], &self.context) {
            return;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);

        self.expert_rebuild_nodes_(v + 1, l, m, should_rebuild);
        self.expert_rebuild_nodes_(vr, m, r, should_rebuild);

        self.pull(v, vr);
    }

    pub fn update_context(&mut self, f: impl Fn(&mut T::Context)) {
        f(&mut self.context);
    }

    pub fn get_context(&self) -> &T::Context {
        &self.context
    }
}
