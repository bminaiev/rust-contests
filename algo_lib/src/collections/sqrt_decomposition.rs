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
        let blocks = gen_vec((n + block_size - 1) / block_size, |id| {
            build_f(id * block_size..min((id + 1) * block_size, n))
        });
        Self {
            n,
            block_size,
            blocks,
        }
    }

    pub fn iter_mut(&mut self, range: Range<usize>, mut f: impl FnMut(Part<T>)) {
        let first_block = range.start / self.block_size;
        let last_block = (range.end + self.block_size - 1) / self.block_size;
        for id in first_block..last_block {
            let cur_block = id * self.block_size..min(self.n, (id + 1) * self.block_size);
            let range = range_intersect(cur_block.clone(), range.clone());
            if range == cur_block {
                f(Part::Full(&mut self.blocks[id]));
            } else {
                self.blocks[id].relax();
                f(Part::Range(
                    &mut self.blocks[id],
                    range.shift_left(id * self.block_size),
                ));
                self.blocks[id].rebuild();
            }
        }
    }
}
