use std::ops::{BitOrAssign, Not};

#[derive(Hash, Clone, Eq, PartialOrd, PartialEq, Debug)]
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

    fn ensure_length(&mut self, bit_len: usize) {
        let i64_len = 1 + (bit_len - 1) >> 6;
        if i64_len > self.values.len() {
            self.values.resize(i64_len, 0);
        }
    }

    fn bit_len(&self) -> usize {
        self.values.len() << 6
    }

    pub fn first_not_set(&self, mut pos: usize) -> usize {
        if pos >= self.bit_len() {
            return pos;
        }
        while (pos & 63) != 0 {
            if !self.get(pos) {
                return pos;
            }
            pos += 1;
        }
        match self.values[pos >> 6..]
            .iter()
            .position(|x| *x != std::u64::MAX)
        {
            None => self.values.len() << 6,
            Some(idx) => {
                pos += idx * 64;
                while self.get(pos) {
                    pos += 1;
                }
                return pos;
            }
        }
    }
}

impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.ensure_length(rhs.bit_len());
        let len = rhs.values.len();
        for (x, y) in self.values[0..len]
            .iter_mut()
            .zip(rhs.values[0..len].iter())
        {
            *x |= *y;
        }
    }
}
