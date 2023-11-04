use std::{collections::BTreeMap, ops::Range};

/// Stores a set of non-overlaping intervals.
///
/// When inserting or removing an interval, the function `f` is called for each
/// new interval that is added or removed from the set.
///
/// If some subinterval of the inserted interval is already in the set, the
/// function `f` is NOT called for that subinterval.
#[derive(Default)]
pub struct IntervalsSet<T: Ord + Clone> {
    right_end: BTreeMap<T, T>,
}

impl<T: Ord + Clone + Copy> IntervalsSet<T> {
    fn split(&mut self, pos: T) {
        if let Some((&l, &r)) = self.right_end.range(..pos).next_back() {
            if r > pos {
                self.right_end.remove(&l);
                self.right_end.insert(l, pos);
                self.right_end.insert(pos, r);
            }
        }
    }

    pub fn remove(&mut self, r: Range<T>, mut f: impl FnMut(Range<T>)) {
        self.split(r.start);
        self.split(r.end);
        while let Some((&l, &r)) = self.right_end.range(r.clone()).next() {
            f(l..r);
            self.right_end.remove(&l);
        }
    }

    pub fn insert(&mut self, r: Range<T>, mut f: impl FnMut(Range<T>)) {
        self.split(r.start);
        self.split(r.end);
        let mut next_seg_from = r.start;
        while let Some((&l, &r)) = self.right_end.range(r.clone()).next() {
            if next_seg_from < l {
                f(next_seg_from..l);
            }
            next_seg_from = r;
            self.right_end.remove(&l);
        }
        if next_seg_from < r.end {
            f(next_seg_from..r.end);
        }
        self.right_end.insert(r.start, r.end);
    }
}
