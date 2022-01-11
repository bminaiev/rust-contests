use crate::collections::array_2d::Array2D;
use crate::misc::bits::index_of_highest_set_bit;
use std::fmt::Debug;
use std::ops::Range;

pub struct SparseTableMax<T>
where
    T: Ord + Clone,
{
    max: Array2D<u32>,
    values: Vec<T>,
}

impl<T> SparseTableMax<T>
where
    T: Ord + Clone + Debug,
{
    fn max_by_pos(values: &[T], pos1: u32, pos2: u32) -> u32 {
        if values[pos1 as usize] > values[pos2 as usize] {
            pos1
        } else {
            pos2
        }
    }

    pub fn new(values: &[T]) -> Self {
        let n = values.len();
        let levels = index_of_highest_set_bit(n) + 1;
        let mut max = Array2D::new(0, levels, n);
        for pos in 0..n {
            max[0][pos] = pos as u32;
        }
        for lvl in 1..levels {
            for pos in 0..n {
                let from = pos + (1 << (lvl - 1));
                if from >= n {
                    max[lvl][pos] = max[lvl - 1][pos];
                } else {
                    max[lvl][pos] = Self::max_by_pos(values, max[lvl - 1][pos], max[lvl - 1][from]);
                }
            }
        }
        Self {
            values: values.iter().cloned().collect(),
            max,
        }
    }

    pub fn find_max_pos(&self, range: Range<usize>) -> usize {
        let len = range.len();
        let lvl = index_of_highest_set_bit(len);
        let from = range.end - (1 << lvl);
        debug_assert!(from >= range.start);
        let res = Self::max_by_pos(
            &self.values,
            self.max[lvl][range.start],
            self.max[lvl][from],
        ) as usize;
        debug_assert!(res >= range.start);
        debug_assert!(
            res < range.end,
            "{:?}, res = {}, values = {:?}, lvl = {}",
            range,
            res,
            self.values,
            lvl
        );
        res
    }
}
