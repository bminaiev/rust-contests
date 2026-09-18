use std::ops::Range;

use super::seg_tree_trait::SegTreeNode;

/// Iterative lazy segment tree using the same nodes and update composition as `SegTree`.
///
/// Joins must be associative, and applying an update must distribute over a join.
/// `join_updates(old, new)` must represent applying `old` followed by `new`.
/// Neither nodes nor updates need to be `Copy`, and `Default` need not be an identity.
/// Ranges are half-open and must be within `0..len()`; empty queries return `T::default()`.
///
/// Uses a power-of-two leaf base, `2 * base` node slots and `base` lazy slots.
/// Traversals allocate no memory and take O(log n); whole-array queries take O(1).
/// Queries do not mutate the tree: pending tags are applied to the query's
/// accumulators instead of being pushed to children. Node operations themselves
/// may allocate or cost more than O(1), depending on the `SegTreeNode` implementation.
/// Unlike `SegTree`, this does not expose expert node indices or predicate searches.
#[derive(Clone)]
pub struct OptimizedSegTree<T: SegTreeNode> {
    n: usize,
    base: usize,
    height: u32,
    tree: Vec<T>,
    lazy: Vec<Option<T::Update>>,
    // Only ancestors of the final real leaf can have a missing right child.
    missing_right: Vec<bool>,
    context: T::Context,
}

impl<T: SegTreeNode> OptimizedSegTree<T> {
    pub fn new(n: usize, f: impl Fn(usize) -> T) -> Self
    where
        T::Context: Default,
    {
        Self::new_with_context(n, f, T::Context::default())
    }

    pub fn new_with_context(n: usize, f: impl Fn(usize) -> T, context: T::Context) -> Self {
        assert!(n > 0);
        let base = n.next_power_of_two();
        let mut res = Self {
            n,
            base,
            height: base.trailing_zeros(),
            tree: vec![T::default(); 2 * base],
            lazy: vec![None; base],
            missing_right: vec![false; base],
            context,
        };
        for i in 0..n {
            res.tree[base + i] = f(i);
        }
        let mut last = base + n - 1;
        while last > 1 {
            res.missing_right[last / 2] = last & 1 == 0;
            last >>= 1;
        }
        let mut first = base >> 1;
        let mut last = (base + n - 1) >> 1;
        while first > 0 {
            for v in first..=last {
                res.pull(v);
            }
            first >>= 1;
            last >>= 1;
        }
        res
    }

    #[inline(always)]
    fn pull(&mut self, v: usize) {
        self.tree[v] = if self.missing_right[v] {
            self.tree[2 * v].clone()
        } else {
            T::join_nodes(&self.tree[2 * v], &self.tree[2 * v + 1], &self.context)
        };
    }

    #[inline(always)]
    fn apply(&mut self, v: usize, update: &T::Update) {
        T::apply_update(&mut self.tree[v], update);
        if v < self.base {
            match &mut self.lazy[v] {
                Some(old) => T::join_updates(old, update),
                slot @ None => *slot = Some(update.clone()),
            }
        }
    }

    #[inline(always)]
    fn push(&mut self, v: usize) {
        if let Some(update) = self.lazy[v].take() {
            self.apply(2 * v, &update);
            self.apply(2 * v + 1, &update);
        }
    }

    // Only partially covered ancestors need pushing. Shared ancestors are visited once.
    #[inline]
    fn push_boundaries(&mut self, l: usize, r: usize) {
        let l_skip = l.trailing_zeros();
        let r_skip = r.trailing_zeros();
        for level in ((l_skip.min(r_skip) + 1)..=self.height).rev() {
            let left = l >> level;
            let right = (r - 1) >> level;
            if level > l_skip {
                self.push(left);
            }
            if level > r_skip && (left != right || level <= l_skip) {
                self.push(right);
            }
        }
    }

    #[inline]
    pub fn update(&mut self, range: Range<usize>, update: T::Update) {
        if range.is_empty() {
            return;
        }
        assert!(range.end <= self.n);
        let mut l = range.start + self.base;
        let mut r = range.end + self.base;
        self.push_boundaries(l, r);
        // Rebuild the two boundary paths while climbing, immediately before
        // processing their siblings. Once they meet, rebuild their shared path.
        let mut left_dirty = false;
        let mut right_dirty = false;
        while l < r {
            if left_dirty {
                self.pull(l - 1);
            }
            if right_dirty {
                self.pull(r);
            }
            if l & 1 != 0 {
                self.apply(l, &update);
                l += 1;
                left_dirty = true;
            }
            if r & 1 != 0 {
                r -= 1;
                self.apply(r, &update);
                right_dirty = true;
            }
            l >>= 1;
            r >>= 1;
        }
        let mut left = if left_dirty { l - 1 } else { 0 };
        let mut right = if right_dirty { r } else { 0 };
        while left != 0 || right != 0 {
            if left != 0 {
                self.pull(left);
            }
            if right != 0 && right != left {
                self.pull(right);
            }
            left >>= 1;
            right >>= 1;
        }
    }

    #[inline]
    pub fn get(&self, range: Range<usize>) -> T {
        if range.is_empty() {
            return T::default();
        }
        assert!(range.end <= self.n);
        if range.start == 0 && range.end == self.n {
            return self.tree[1].clone();
        }
        let mut l = range.start + self.base;
        let mut r = range.end + self.base;
        // Each accumulator holds the part already consumed on its boundary.
        // On ascent it receives that ancestor's pending update before joining
        // the next fully covered sibling. Ancestor tags are always newer than
        // descendant tags, because updates push before descending.
        // Separate accumulators preserve order for noncommutative joins. Options
        // avoid requiring Default to be an identity (e.g. max of negative values).
        let mut left = None;
        let mut right = None;
        while l < r {
            if let Some(ref mut value) = left {
                if let Some(update) = &self.lazy[l - 1] {
                    T::apply_update(value, update);
                }
            }
            if let Some(ref mut value) = right {
                if let Some(update) = &self.lazy[r] {
                    T::apply_update(value, update);
                }
            }
            if l & 1 != 0 {
                left = Some(match left {
                    None => self.tree[l].clone(),
                    Some(ref old) => T::join_nodes(old, &self.tree[l], &self.context),
                });
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                right = Some(match right {
                    None => self.tree[r].clone(),
                    Some(ref old) => T::join_nodes(&self.tree[r], old, &self.context),
                });
            }
            l >>= 1;
            r >>= 1;
        }
        // The boundary fragments may still be in different sibling subtrees.
        // Apply their ancestors' tags before joining, then apply shared tags once.
        let mut lv = l - 1;
        let mut rv = r;
        while lv != rv {
            if let Some(ref mut value) = left {
                if let Some(update) = &self.lazy[lv] {
                    T::apply_update(value, update);
                }
            }
            if let Some(ref mut value) = right {
                if let Some(update) = &self.lazy[rv] {
                    T::apply_update(value, update);
                }
            }
            lv >>= 1;
            rv >>= 1;
        }
        let mut result = match (left, right) {
            (Some(l), Some(r)) => T::join_nodes(&l, &r, &self.context),
            (Some(v), None) | (None, Some(v)) => v,
            _ => unreachable!(),
        };
        while lv > 0 {
            if let Some(update) = &self.lazy[lv] {
                T::apply_update(&mut result, update);
            }
            lv >>= 1;
        }
        result
    }

    pub fn update_point(&mut self, pos: usize, new_node: T) {
        assert!(pos < self.n);
        let v = self.base + pos;
        for level in (1..=self.height).rev() {
            self.push(v >> level);
        }
        self.tree[v] = new_node;
        for level in 1..=self.height {
            self.pull(v >> level);
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn get_context(&self) -> &T::Context {
        &self.context
    }

    /// As with `SegTree`, changing context does not rebuild existing aggregates.
    pub fn update_context(&mut self, f: impl Fn(&mut T::Context)) {
        f(&mut self.context);
    }
}
