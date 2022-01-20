use crate::misc::gen_vector::gen_vec;
use crate::misc::num_traits::Number;
use std::ops::Range;

#[allow(dead_code)]
pub struct Random {
    state: u64,
}

impl Random {
    pub fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[allow(dead_code)]
    pub fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        (from as u64 + self.next() % ((to - from) as u64)) as usize
    }

    #[allow(dead_code)]
    pub fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (usize::MAX as f64)
    }

    #[allow(dead_code)]
    pub fn new(seed: u64) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }

    #[allow(dead_code)]
    pub fn next_permutation(&mut self, n: usize) -> Vec<usize> {
        let mut result: Vec<_> = (0..n).collect();
        for i in 0..n {
            let idx = self.next_in_range(0, i + 1);
            result.swap(i, idx);
        }
        result
    }

    pub fn gen_range<T>(&mut self, range: Range<T>) -> T
    where
        T: Number,
    {
        let from = T::to_i32(range.start);
        let to = T::to_i32(range.end);
        let len = (to - from) as usize;
        T::from_i32(self.next_in_range(0, len) as i32 + from)
    }

    pub fn gen_vec<T>(&mut self, n: usize, range: Range<T>) -> Vec<T>
    where
        T: Number,
    {
        gen_vec(n, |_| self.gen_range(range.clone()))
    }

    pub fn gen_nonempty_range(&mut self, n: usize) -> Range<usize> {
        let x = self.gen_range(0..n);
        let y = self.gen_range(0..n);
        if x <= y {
            x..y + 1
        } else {
            y..x + 1
        }
    }

    pub fn gen_bool(&mut self) -> bool {
        self.gen_range(0..2) == 0
    }
}
