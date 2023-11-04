use crate::{collections::fx_hash_map::FxHashMap, misc::num_traits::Number};

pub struct LazyFenwick<T> {
    data: FxHashMap<i32, T>,
    n: i32,
}

impl<T: Number> LazyFenwick<T> {
    pub fn new(n: i32) -> Self {
        Self {
            data: FxHashMap::default(),
            n,
        }
    }

    pub fn get_sum(&self, mut pos: i32) -> T {
        let mut res = T::ZERO;
        loop {
            res += self.data.get(&pos).copied().unwrap_or_default();
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    pub fn add(&mut self, mut pos: i32, change: T) {
        while pos < self.n {
            *self.data.entry(pos).or_default() += change;
            pos |= pos + 1;
        }
    }
}
