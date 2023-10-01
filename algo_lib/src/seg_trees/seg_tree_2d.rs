use std::ops::{Range, RangeInclusive};

use crate::{
    misc::binary_search::binary_search_first_true,
    seg_trees::{lazy_seg_tree::SegTree, seg_tree_trait::SegTreeNode},
};

pub struct SegTree2d<T: Ord, U: SegTreeNode> {
    all_y: Vec<T>,
    tree_y: SegTree<U>,
    x_range: RangeInclusive<T>,
    mid_x: T,
    child: Vec<SegTree2d<T, U>>,
}

impl<T: Ord + Copy, U: SegTreeNode> SegTree2d<T, U> {
    pub fn new(pts: Vec<(T, T)>) -> Self
    where
        U::Context: Default,
    {
        assert!(!pts.is_empty());
        let mut all_x: Vec<T> = pts.iter().map(|&(x, _y)| x).collect();
        all_x.sort();
        all_x.dedup();
        let mut all_y: Vec<T> = pts.iter().map(|&(_x, y)| y).collect();
        all_y.sort();
        all_y.dedup();
        let tree_y = SegTree::new(all_y.len(), |_| U::default());
        let mid_x = all_x[all_x.len() / 2];
        let mut child = vec![];
        if all_x.len() > 1 {
            let mut left_pts = vec![];
            let mut right_pts = vec![];
            for &(x, y) in pts.iter() {
                if x < mid_x {
                    left_pts.push((x, y));
                } else {
                    right_pts.push((x, y));
                }
            }
            assert!(!left_pts.is_empty() && !right_pts.is_empty());
            let left = Self::new(left_pts);
            let right = Self::new(right_pts);
            child.push(left);
            child.push(right);
        }
        Self {
            all_y,
            tree_y,
            mid_x,
            child,
            x_range: all_x[0]..=all_x[all_x.len() - 1],
        }
    }

    pub fn update(&mut self, x: T, y: T, value: U) {
        let y_pos = self.all_y.binary_search(&y).unwrap();
        self.tree_y.update_point(y_pos, value.clone());
        if !self.child.is_empty() {
            if x < self.mid_x {
                self.child[0].update(x, y, value);
            } else {
                self.child[1].update(x, y, value);
            }
        }
    }

    pub fn query(&mut self, x_range: Range<T>, y_range: Range<T>) -> U {
        let mut res = U::default();

        if x_range.start <= *self.x_range.start() && x_range.end > *self.x_range.end() {
            let y_start =
                binary_search_first_true(0..self.all_y.len(), |p| self.all_y[p] >= y_range.start);
            let y_end =
                binary_search_first_true(0..self.all_y.len(), |p| self.all_y[p] >= y_range.end);
            res = self.tree_y.get(y_start..y_end);
        } else if !self.child.is_empty() {
            if x_range.start < self.mid_x {
                res = U::join_nodes(
                    &res,
                    &self.child[0].query(x_range.clone(), y_range.clone()),
                    self.tree_y.get_context(),
                );
            }
            if x_range.end > self.mid_x {
                res = U::join_nodes(
                    &res,
                    &self.child[1].query(x_range.clone(), y_range.clone()),
                    self.tree_y.get_context(),
                );
            }
        }
        res
    }
}
