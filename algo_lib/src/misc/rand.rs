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
}
