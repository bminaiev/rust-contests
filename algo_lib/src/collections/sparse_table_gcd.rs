use crate::collections::sparse_table::{SparseTable, SparseTableSpec};
use crate::math::gcd::gcd;
use crate::misc::num_traits::Number;
use std::marker::PhantomData;

pub struct SparseTableGcdSpec<T> {
    phantom: PhantomData<T>,
}

impl<T> SparseTableSpec for SparseTableGcdSpec<T>
where
    T: Number + std::ops::Rem<Output = T>,
{
    type Element = T;
    type Result = T;

    fn convert(_pos: usize, elem: &Self::Element) -> Self::Result {
        *elem
    }

    fn join(lhs: &Self::Result, rhs: &Self::Result, _elements: &[Self::Element]) -> Self::Result {
        gcd(*lhs, *rhs)
    }
}

pub type SparseTableGCD<T> = SparseTable<SparseTableGcdSpec<T>>;
