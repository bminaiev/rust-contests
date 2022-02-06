use std::ops::Not;

#[derive(Hash, Clone, Eq, PartialOrd, PartialEq)]
pub struct BitSet {
    values: Vec<u64>,
}

impl BitSet {
    #[allow(unused)]
    pub fn new(n: usize) -> Self {
        Self {
            values: vec![0u64; (n + 63) / 64],
        }
    }

    #[allow(unused)]
    pub fn get(&self, pos: usize) -> bool {
        (self.values[pos >> 6] >> (pos & 63)) & 1 == 1
    }

    #[allow(unused)]
    pub fn set(&mut self, pos: usize, val: bool) {
        if val {
            self.values[pos >> 6] |= 1u64 << (pos & 63);
        } else {
            self.values[pos >> 6] &= (1u64 << (pos & 63)).not();
        }
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        for x in self.values.iter_mut() {
            *x = 0;
        }
    }
}
