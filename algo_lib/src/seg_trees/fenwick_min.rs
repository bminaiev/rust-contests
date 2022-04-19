use crate::misc::{min_max::UpdateMinMax, num_traits::Number};

#[allow(dead_code)]
#[derive(Clone)]
pub struct FenwickMin<T: Number> {
    values: Vec<T>,
}

impl<T: Number> FenwickMin<T> {
    #[allow(dead_code)]
    pub fn get_range_max(&self, mut pos: usize) -> T {
        let mut res = T::MAX;
        loop {
            res.update_min(self.values[pos]);
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, mut pos: usize, change: T) {
        while pos < self.values.len() {
            self.values[pos].update_min(change);
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let values = vec![T::MAX; n];
        FenwickMin { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
