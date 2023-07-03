use std::ops::Range;

use crate::seg_trees::kenetic_seg_tree::{KeneticLine, KeneticSegTreeMin};

pub struct KeneticSegTreeMax {
    tree: KeneticSegTreeMin,
}

impl KeneticSegTreeMax {
    pub fn new(n: usize, f: impl Fn(usize) -> (i64, i64), time: i64) -> Self {
        Self {
            tree: KeneticSegTreeMin::new(
                n,
                |pos| {
                    let (a, b) = f(pos);
                    (-a, -b)
                },
                time,
            ),
        }
    }

    pub fn update(&mut self, pos: usize, a: i64, b: i64) {
        self.tree.update(pos, -a, -b);
    }

    pub fn get_max(&mut self, range: Range<usize>) -> KeneticLine {
        let line = self.tree.get_min(range);
        KeneticLine {
            a: -line.a,
            b: -line.b,
            pos: line.pos,
        }
    }

    pub fn update_time(&mut self, new_time: i64) {
        self.tree.update_time(new_time);
    }
}
