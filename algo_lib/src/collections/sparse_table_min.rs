use crate::collections::sparse_table_max::SparseTableMax;
use std::cmp::Reverse;
use std::fmt::Debug;
use std::ops::Range;

#[derive(Debug)]
pub struct SparseTableMin<T>(SparseTableMax<Reverse<T>>)
where
    T: Clone + Ord + Debug;

impl<T> SparseTableMin<T>
where
    T: Clone + Ord + Debug,
{
    pub fn new(values: &[T]) -> Self {
        let values_rev: Vec<_> = values.iter().cloned().map(|x| Reverse(x)).collect();
        Self(SparseTableMax::new(&values_rev))
    }

    pub fn find_min_pos(&self, range: Range<usize>) -> usize {
        self.0.find_max_pos(range)
    }
}
