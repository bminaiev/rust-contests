use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, Not};

#[derive(Hash, Clone, Eq, PartialOrd, PartialEq, Debug)]
pub struct BitSet {
    values: Vec<u64>,
}

impl BitSet {
    pub fn calc_len(n: usize) -> usize {
        (n + 127) / 128 * 2
    }

    #[allow(unused)]
    pub fn new(n: usize) -> Self {
        Self {
            values: vec![0u64; BitSet::calc_len(n)],
        }
    }

    #[allow(unused)]
    pub fn get(&self, pos: usize) -> bool {
        (self.values[pos >> 6] >> (pos & 63)) & 1 == 1
    }

    pub fn get_u64(&self, from_pos: usize) -> u64 {
        if from_pos >= self.bit_len() {
            return 0;
        }
        if from_pos & 63 == 0 {
            self.values[from_pos >> 6]
        } else {
            let mut res = self.values[from_pos >> 6] >> (from_pos & 63);
            if from_pos + 64 < self.bit_len() {
                res |= self.values[(from_pos >> 6) + 1] << (64 - (from_pos & 63))
            }
            res
        }
    }

    // bit [i] becomes [i + shift]. Bits after [n] are almost dropped.
    pub fn shift_higher(&self, shift: usize) -> Self {
        let mut res = Self::new(self.bit_len());
        let whole = shift / 64;
        let offset = shift % 64;
        for i in 0..self.values.len() {
            if i + whole >= res.values.len() {
                break;
            }
            res.values[i + whole] |= self.values[i] << offset;
            if offset != 0 && i + whole + 1 < res.values.len() {
                res.values[i + whole + 1] |= self.values[i] >> (64 - offset);
            }
        }
        res
    }

    // bit [i] becomes [i - shift]. Bits before [0] are dropped.
    pub fn shift_lower(&self, shift: usize) -> Self {
        let mut res = Self::new(self.bit_len());
        let whole = shift / 64;
        let offset = shift % 64;
        for i in 0..self.values.len() {
            if i < whole {
                continue;
            }
            // TODO: test
            res.values[i - whole] |= self.values[i] >> offset;
            if offset != 0 && i - whole != 0 {
                res.values[i - whole - 1] |= self.values[i] << (64 - offset);
            }
        }
        res
    }

    #[allow(unused)]
    pub fn set(&mut self, pos: usize, val: bool) {
        if val {
            self.values[pos >> 6] |= 1u64 << (pos & 63);
        } else {
            self.values[pos >> 6] &= (1u64 << (pos & 63)).not();
        }
    }

    pub fn set_true(&mut self, pos: usize) {
        self.values[pos >> 6] |= 1u64 << (pos & 63);
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        for x in self.values.iter_mut() {
            *x = 0;
        }
    }

    fn ensure_length(&mut self, bit_len: usize) {
        let i64_len = Self::calc_len(bit_len);
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
                pos
            }
        }
    }

    pub fn first_set(&self, mut pos: usize) -> Option<usize> {
        if pos >= self.bit_len() {
            return None;
        }
        if (pos & 63) != 0 {
            let part = self.values[pos >> 6] >> (pos & 63);
            if part != 0 {
                return Some(pos + part.trailing_zeros() as usize);
            }
            pos = (pos | 63) + 1;
        }
        match self.values[pos >> 6..].iter().position(|x| *x != 0) {
            None => None,
            Some(idx) => {
                pos += idx * 64;
                pos += self.values[pos >> 6].trailing_zeros() as usize;
                assert!(self.get(pos));
                Some(pos)
            }
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn bitor_assign_avx2(&mut self, rhs: &Self) {
        for (x, y) in self.values.iter_mut().zip(rhs.values.iter()) {
            *x |= *y;
        }
    }

    #[target_feature(enable = "ssse3")]
    unsafe fn bitor_assign_ssse3(&mut self, rhs: &Self) {
        for (x, y) in self.values.iter_mut().zip(rhs.values.iter()) {
            *x |= *y;
        }
    }

    pub fn count_ones(&self) -> usize {
        self.values.iter().map(|x| x.count_ones() as usize).sum()
    }
}

impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.ensure_length(rhs.bit_len());
        if is_x86_feature_detected!("avx2") {
            unsafe {
                self.bitor_assign_avx2(rhs);
            }
        } else if is_x86_feature_detected!("ssse3") {
            unsafe {
                self.bitor_assign_ssse3(rhs);
            }
        } else {
            for (x, y) in self.values.iter_mut().zip(rhs.values.iter()) {
                *x |= *y;
            }
        }
    }
}

impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &BitSet) {
        self.ensure_length(rhs.bit_len());
        let len = rhs.values.len();
        for (x, y) in self.values[0..len]
            .iter_mut()
            .zip(rhs.values[0..len].iter())
        {
            *x &= *y;
        }
    }
}

impl BitXorAssign<&BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &BitSet) {
        self.ensure_length(rhs.bit_len());
        let len = rhs.values.len();
        for (x, y) in self.values[0..len]
            .iter_mut()
            .zip(rhs.values[0..len].iter())
        {
            *x ^= *y;
        }
    }
}
