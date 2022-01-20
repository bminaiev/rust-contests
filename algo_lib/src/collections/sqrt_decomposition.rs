use crate::misc::gen_vector::gen_vec;
use crate::misc::range_intersect::range_intersect;
use std::cmp::min;
use std::ops::Range;

pub trait SqrtNode: Clone {
    type Value: Clone;

    fn relax(&mut self, raw_values: &mut [Self::Value]);
    fn rebuild(&mut self, raw_values: &[Self::Value]);
}

pub struct SqrtDecomposition<T>
where
    T: SqrtNode,
{
    raw_values: Vec<T::Value>,
    block_size: usize,
    blocks: Vec<T>,
}

// TODO: think about better name?
pub enum Part<'a, T>
where
    T: SqrtNode,
{
    Full(&'a mut T),
    Single(&'a mut T, &'a mut T::Value),
}

impl<T> SqrtDecomposition<T>
where
    T: SqrtNode,
{
    pub fn new(raw_values: Vec<T::Value>, block_size: usize, empty_block: T) -> Self {
        assert!(block_size > 0);
        let n = raw_values.len();
        let blocks_num = (n + block_size - 1) / block_size;
        let blocks = gen_vec(blocks_num, |id| {
            let mut block = empty_block.clone();
            block.rebuild(&raw_values[id * block_size..min((id + 1) * block_size, n)]);
            block
        });
        Self {
            raw_values,
            block_size,
            blocks,
        }
    }

    pub fn iter_mut<F>(&mut self, range: Range<usize>, mut f: F)
    where
        F: FnMut(Part<T>),
    {
        let first_block = range.start / self.block_size;
        let last_block = (range.end + self.block_size - 1) / self.block_size;
        let block_size = self.block_size;

        let handle_side_block =
            |id: usize, f: &mut F, block: &mut T, raw_values: &mut [T::Value]| {
                let n = raw_values.len();
                let cur_block = block_size * id..min(n, block_size * (id + 1));
                let range = range_intersect(cur_block.clone(), range.clone());
                if range == cur_block {
                    f(Part::Full(block));
                } else {
                    block.relax(&mut raw_values[cur_block.clone()]);
                    for single in raw_values[range].iter_mut() {
                        f(Part::Single(block, single));
                    }
                    block.rebuild(&raw_values[cur_block]);
                }
            };

        handle_side_block(
            first_block,
            &mut f,
            &mut self.blocks[first_block],
            &mut self.raw_values,
        );
        if first_block + 1 < last_block {
            for block_id in first_block + 1..last_block - 1 {
                f(Part::Full(&mut self.blocks[block_id]))
            }
            handle_side_block(
                last_block - 1,
                &mut f,
                &mut self.blocks[last_block - 1],
                &mut self.raw_values,
            );
        }
    }
}
