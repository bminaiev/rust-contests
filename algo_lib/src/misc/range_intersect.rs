use crate::misc::num_traits::Number;
use std::cmp::{max, min};
use std::ops::Range;

pub fn range_intersect(r1: Range<usize>, r2: Range<usize>) -> Range<usize> {
    max(r1.start, r2.start)..min(r1.end, r2.end)
}

pub trait Shift {
    fn shift<T>(self, delta: T) -> Self
    where
        T: Number;

    fn shift_left<T>(self, delta: T) -> Self
    where
        T: Number;
}

impl Shift for Range<usize> {
    fn shift<T>(self, delta: T) -> Self
    where
        T: Number,
    {
        let start = (self.start as i32 + delta.to_i32()) as usize;
        let end = (self.end as i32 + delta.to_i32()) as usize;
        start..end
    }

    fn shift_left<T>(self, delta: T) -> Self
    where
        T: Number,
    {
        self.shift(-delta.to_i32())
    }
}
