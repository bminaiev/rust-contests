// partially copied from https://github.com/EgorKulikov/rust_algo/blob/master/algo_lib/src/numbers/prime_fft.rs
// article: https://cp-algorithms.com/algebra/fft.html#:~:text=and%20their%20powers.-,Number%20theoretic%20transform,-Now%20we%20switch

use crate::math::modulo::{ModWithValue, ModuloTrait, Value};
use crate::misc::num_traits::ConvI32;

pub struct NTT<M>
where
    M: ModuloTrait,
{
    root: M,
    root_power: usize,
}

impl<M> NTT<M>
where
    M: ModuloTrait,
{
    pub fn new() -> Self {
        let root_power = 1 << (M::mod_value() - 1).trailing_zeros();

        let root = (2..)
            .map(M::from_i32)
            .filter(|&root| -> bool {
                root.pown(root_power - 1) != M::ONE && root.pown(root_power) == M::ONE
            })
            .next()
            .unwrap();

        Self { root, root_power }
    }

    pub fn multiply(&self, mut a: Vec<M>, mut b: Vec<M>) -> Vec<M> {
        let result_size = a.len() + b.len() - 1;
        let expected_size = (result_size).next_power_of_two();
        a.resize(expected_size, M::ZERO);
        b.resize(expected_size, M::ZERO);
        self.fft(&mut a, false);
        self.fft(&mut b, false);
        for (x, &y) in a.iter_mut().zip(b.iter()) {
            *x *= y;
        }
        self.fft(&mut a, true);
        a.truncate(result_size);
        a
    }

    fn multiply_all_ref(&self, polynomials: &mut [Vec<M>]) -> Vec<M> {
        if polynomials.len() == 1 {
            polynomials[0].to_vec()
        } else {
            let mid = polynomials.len() / 2;
            let (lhs, rhs) = polynomials.split_at_mut(mid);
            let lhs = self.multiply_all_ref(lhs);
            let rhs = self.multiply_all_ref(rhs);
            self.multiply(lhs, rhs)
        }
    }

    pub fn multiply_all(&self, mut polynomials: Vec<Vec<M>>) -> Vec<M> {
        self.multiply_all_ref(&mut polynomials)
    }

    fn fft(&self, a: &mut [M], invert: bool) {
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
                M::ONE / self.root
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
                let mut w = M::ONE;
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
            let inv_size = M::ONE / M::from_i32(a.len() as i32);
            for i in a {
                *i *= inv_size;
            }
        }
    }
}
