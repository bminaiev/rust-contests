use crate::collections::array_2d::Array2D;
use crate::misc::bits::index_of_highest_set_bit;
use std::fmt::Debug;
use std::ops::Range;

pub trait SparseTableSpec {
    type Element: Clone + Debug;
    type Result: Clone + Debug + Default;

    fn convert(pos: usize, elem: &Self::Element) -> Self::Result;
    fn join(lhs: &Self::Result, rhs: &Self::Result, elements: &[Self::Element]) -> Self::Result;
}

#[derive(Debug)]
pub struct SparseTable<Spec>
where
    Spec: SparseTableSpec,
{
    matrix: Array2D<Spec::Result>,
    values: Vec<Spec::Element>,
}

impl<Spec> SparseTable<Spec>
where
    Spec: SparseTableSpec,
{
    pub fn new(values: &[Spec::Element]) -> Self {
        let n = values.len();
        let levels = index_of_highest_set_bit(n) + 1;
        let mut matrix = Array2D::new(Spec::Result::default(), levels, n);
        for pos in 0..n {
            matrix[0][pos] = Spec::convert(pos, &values[pos]);
        }
        for lvl in 1..levels {
            for pos in 0..n {
                let from = pos + (1 << (lvl - 1));
                if from >= n {
                    matrix[lvl][pos] = matrix[lvl - 1][pos].clone();
                } else {
                    matrix[lvl][pos] =
                        Spec::join(&matrix[lvl - 1][pos], &matrix[lvl - 1][from], &values);
                }
            }
        }
        Self {
            values: values.to_vec(),
            matrix,
        }
    }

    pub fn query(&self, range: Range<usize>) -> Spec::Result {
        let len = range.len();
        assert!(len > 0);
        let lvl = index_of_highest_set_bit(len);
        let from = range.end - (1 << lvl);
        debug_assert!(from >= range.start);
        Spec::join(
            &self.matrix[lvl][range.start],
            &self.matrix[lvl][from],
            &self.values,
        )
    }
}
