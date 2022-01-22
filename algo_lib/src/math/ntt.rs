// partially copied from https://github.com/EgorKulikov/rust_algo/blob/master/algo_lib/src/numbers/prime_fft.rs
// article: https://cp-algorithms.com/algebra/fft.html#:~:text=and%20their%20powers.-,Number%20theoretic%20transform,-Now%20we%20switch

use crate::math::modulo::{ModWithValue, Value};
use crate::misc::num_traits::ConvI32;

pub struct NTT<M>
where
    M: Value,
{
    root: ModWithValue<M>,
    root_power: usize,
}

impl<M> NTT<M>
where
    M: Value,
{
    pub fn new() -> Self {
        let root_power = 1 << (M::val() - 1).trailing_zeros();

        let root = (2..)
            .map(ModWithValue::<M>::new)
            .filter(|&root| -> bool {
                root.pown(root_power - 1) != ModWithValue::<M>::ONE
                    && root.pown(root_power) == ModWithValue::<M>::ONE
            })
            .next()
            .unwrap();

        Self { root, root_power }
    }

    pub fn multiply(
        &self,
        mut a: Vec<ModWithValue<M>>,
        mut b: Vec<ModWithValue<M>>,
    ) -> Vec<ModWithValue<M>> {
        let result_size = a.len() + b.len() - 1;
        let expected_size = (result_size).next_power_of_two();
        a.resize(expected_size, ModWithValue::<M>::ZERO);
        b.resize(expected_size, ModWithValue::<M>::ZERO);
        self.fft(&mut a, false);
        self.fft(&mut b, false);
        for (x, &y) in a.iter_mut().zip(b.iter()) {
            *x *= y;
        }
        self.fft(&mut a, true);
        a.truncate(result_size);
        a
    }

    fn fft(&self, a: &mut [ModWithValue<M>], invert: bool) {
        assert!(a.len().is_power_of_two());
        let mut j = 0usize;
        for i in 1..a.len() {
            let mut bit = a.len() >> 1;
            while j >= bit {
                j -= bit;
                bit >>= 1;
            }
            j += bit;
            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;
        while len <= a.len() {
            let mut w_len = if invert {
                ModWithValue::<M>::ONE / self.root
            } else {
                self.root
            };
            let mut i = len;
            while i < self.root_power {
                w_len *= w_len;
                i += i;
            }
            let half = len >> 1;
            for i in (0..a.len()).step_by(len) {
                let mut w = ModWithValue::<M>::ONE;
                for j in 0..half {
                    let u = a[i + j];
                    let v = a[i + j + half] * w;
                    a[i + j] = u + v;
                    a[i + j + half] = u - v;
                    w *= w_len;
                }
            }
            len <<= 1;
        }
        if invert {
            let inv_size = ModWithValue::<M>::ONE / ModWithValue::<M>::from_i32(a.len() as i32);
            for i in a {
                *i *= inv_size;
            }
        }
    }
}
