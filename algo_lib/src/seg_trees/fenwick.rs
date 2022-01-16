use std::ops::Range;

#[allow(dead_code)]
pub struct Fenwick {
    values: Vec<i64>,
}

impl Fenwick {
    #[allow(dead_code)]
    pub fn get_sum(&self, mut pos: usize) -> i64 {
        let mut res = 0i64;
        loop {
            res += self.values[pos] as i64;
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    pub fn get_range_sum(&self, range: Range<usize>) -> i64 {
        if range.end == 0 {
            return 0;
        }
        let res = self.get_sum(range.end - 1);
        if range.start == 0 {
            res
        } else {
            res - self.get_sum(range.start - 1)
        }
    }

    pub fn get_suffix_sum(&self, pos: usize) -> i64 {
        let total = self.get_sum(self.values.len() - 1);
        let before = if pos == 0 { 0 } else { self.get_sum(pos - 1) };
        total - before
    }

    #[allow(dead_code)]
    pub fn add(&mut self, mut pos: usize, change: i64) {
        while pos < self.values.len() {
            self.values[pos] += change;
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let values = vec![0; n];
        Fenwick { values }
    }

    pub fn new_pow2(n: usize) -> Self {
        Self::new(n.next_power_of_two())
    }
}
