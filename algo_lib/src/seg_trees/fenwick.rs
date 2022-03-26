use std::ops::Range;

use crate::misc::num_traits::Number;

#[allow(dead_code)]
pub struct Fenwick<T: Number> {
    values: Vec<T>,
}

impl<T: Number> Fenwick<T> {
    #[allow(dead_code)]
    pub fn get_sum(&self, mut pos: usize) -> T {
        let mut res = T::ZERO;
        loop {
            res += self.values[pos];
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    pub fn get_range_sum(&self, range: Range<usize>) -> T {
        if range.end == 0 {
            return T::ZERO;
        }
        let res = self.get_sum(range.end - 1);
        if range.start == 0 {
            res
        } else {
            res - self.get_sum(range.start - 1)
        }
    }

    pub fn get_suffix_sum(&self, pos: usize) -> T {
        let total = self.get_sum(self.values.len() - 1);
        let before = if pos == 0 {
            T::ZERO
        } else {
            self.get_sum(pos - 1)
        };
        total - before
    }

    #[allow(dead_code)]
    pub fn add(&mut self, mut pos: usize, change: T) {
        while pos < self.values.len() {
            self.values[pos] += change;
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let values = vec![T::ZERO; n];
        Fenwick { values }
    }

    pub fn new_pow2(n: usize) -> Self {
        Self::new(n.next_power_of_two())
    }

    pub fn clear(&mut self) {
        for x in self.values.iter_mut() {
            *x = T::ZERO;
        }
    }
}
