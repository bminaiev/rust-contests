use std::ops::Range;

use crate::{
    misc::min_max::UpdateMinMax,
    seg_trees::lazy_seg_tree::{SegTree, SegTreeNode},
};

const MAX: i64 = std::i64::MAX;

#[derive(Clone, Default)]
pub struct KeneticLine {
    pub a: i64,
    pub b: i64,
    pub pos: usize,
}

struct KeneticLineCmp {
    smaller: KeneticLine,
    change_at: i64,
}

impl KeneticLine {
    pub fn get_value(&self, time: i64) -> i64 {
        if self.b == MAX {
            return MAX;
        }
        self.a * time + self.b
    }

    // precondition: current self is smaller
    fn when_other_be_smaller(&self, other: &Self) -> i64 {
        // a1 * time + b1 > a2 * time + b2
        // (a1 - a2) * time > b2 - b1
        // time > (b2 - b1) / (a1 - a2)
        let denom = self.a - other.a;
        if denom <= 0 || other.b == MAX || self.b == MAX {
            MAX
        } else {
            (other.b - self.b) / denom + 1
        }
    }

    fn cmp(&self, other: &Self, time: i64) -> KeneticLineCmp {
        let self_value = self.get_value(time);
        let other_value = other.get_value(time);
        if self_value <= other_value {
            KeneticLineCmp {
                smaller: self.clone(),
                change_at: self.when_other_be_smaller(other),
            }
        } else {
            KeneticLineCmp {
                smaller: other.clone(),
                change_at: other.when_other_be_smaller(self),
            }
        }
    }
}

#[derive(Clone, Default)]
struct KeneticSegTreeNode {
    min_value: KeneticLine,
    first_time_to_change: i64,
}

impl SegTreeNode for KeneticSegTreeNode {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        let cmp = l.min_value.cmp(&r.min_value, *context);
        let mut res = Self {
            min_value: cmp.smaller,
            first_time_to_change: cmp.change_at,
        };
        res.first_time_to_change.update_min(l.first_time_to_change);
        res.first_time_to_change.update_min(r.first_time_to_change);
        res
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.min_value = KeneticLine {
            a: update.0,
            b: update.1,
            pos: node.min_value.pos,
        }
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = (i64, i64);

    type Context = i64;
}

/// Stores value[i] = a[i] * time + b[i]
/// Allows to find min value[i]
/// `time` could only increase.
pub struct KeneticSegTreeMin {
    tree: SegTree<KeneticSegTreeNode>,
}

impl KeneticSegTreeMin {
    // f(i) = (a[i], b[i])
    pub fn new(n: usize, f: impl Fn(usize) -> (i64, i64), time: i64) -> Self {
        let tree = SegTree::new_with_context(
            n,
            |pos| {
                let (a, b) = f(pos);
                KeneticSegTreeNode {
                    min_value: KeneticLine { a, b, pos },
                    first_time_to_change: MAX,
                }
            },
            time,
        );
        Self { tree }
    }

    pub fn update(&mut self, pos: usize, a: i64, b: i64) {
        self.tree.update(pos..pos + 1, (a, b));
        self.recalc_if_needed();
    }

    pub fn get_min(&mut self, range: Range<usize>) -> KeneticLine {
        self.recalc_if_needed();
        self.tree.get(range).min_value
    }

    pub fn update_time(&mut self, new_time: i64) {
        self.tree.update_context(|ctx| {
            assert!(*ctx <= new_time);
            *ctx = new_time;
        });
        self.recalc_if_needed();
    }

    fn recalc_if_needed(&mut self) {
        self.tree
            .expert_rebuild_nodes(|node, &time| node.first_time_to_change <= time);
    }
}
