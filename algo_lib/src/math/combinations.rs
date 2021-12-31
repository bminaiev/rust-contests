use crate::math::factorials::facts;
use crate::misc::num_traits::Number;

pub trait Combinations<T> {
    fn c(&self, n: usize, k: usize) -> T;
}

pub struct CombinationsFact<T> {
    fact: Vec<T>,
    fact_inv: Vec<T>,
}

impl<T> CombinationsFact<T>
where
    T: Number,
{
    #[allow(unused)]
    pub fn new(n: usize) -> Self {
        let fact = facts(n);
        let mut fact_inv = fact.clone();
        assert_eq!(fact_inv.len(), n + 1);
        fact_inv[n] = T::ONE / fact_inv[n];
        for i in (1..n).rev() {
            fact_inv[i] = fact_inv[i + 1] * T::from_i32((i + 1) as i32);
        }
        Self { fact, fact_inv }
    }
}

impl<T> Combinations<T> for CombinationsFact<T>
where
    T: Number,
{
    fn c(&self, n: usize, k: usize) -> T {
        if k > n {
            return T::ZERO;
        }
        self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
    }
}
