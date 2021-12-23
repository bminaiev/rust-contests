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

    #[allow(dead_code)]
    pub fn add(&mut self, mut pos: usize, change: i64) {
        while pos < self.values.len() {
            self.values[pos] += change;
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new(n: usize) -> Self {
        let values = vec![0; n];
        Fenwick { values }
    }
}
