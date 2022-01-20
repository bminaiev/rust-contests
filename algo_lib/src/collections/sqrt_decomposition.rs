use crate::misc::gen_vector::gen_vec;
use crate::misc::range_intersect::{range_intersect, Shift};
use std::cmp::min;
use std::ops::Range;

pub trait SqrtNode: Clone {
    fn relax(&mut self);
    fn rebuild(&mut self);
}

pub struct SqrtDecomposition<T>
where
    T: SqrtNode,
{
    n: usize,
    block_size: usize,
    blocks: Vec<T>,
}

// TODO: think about better name?
pub enum Part<'a, T> {
    Full(&'a mut T),
    Range(&'a mut T, Range<usize>),
}

impl<T> SqrtDecomposition<T>
where
    T: SqrtNode,
{
    pub fn new(n: usize, block_size: usize, mut build_f: impl FnMut(Range<usize>) -> T) -> Self {
        assert!(block_size > 0);
        let blocks_num = (n + block_size - 1) / block_size;
        let blocks = gen_vec(blocks_num, |id| {
            build_f(id * block_size..min((id + 1) * block_size, n))
        });
        Self {
            n,
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

        let handle_side_block = |id: usize, f: &mut F, block: &mut T| {
            let cur_block = id * self.block_size..min(self.n, (id + 1) * self.block_size);
            let range = range_intersect(cur_block.clone(), range.clone());
            if range == cur_block {
                f(Part::Full(block));
            } else {
                block.relax();
                f(Part::Range(block, range.shift_left(id * self.block_size)));
                block.rebuild();
            }
        };

        handle_side_block(first_block, &mut f, &mut self.blocks[first_block]);
        if first_block + 1 < last_block {
            for block_id in first_block + 1..last_block - 1 {
                f(Part::Full(&mut self.blocks[block_id]))
            }
            handle_side_block(last_block - 1, &mut f, &mut self.blocks[last_block - 1]);
        }
    }
}
