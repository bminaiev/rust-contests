use std::ops::Range;

use crate::seg_trees::kinetic_seg_tree::{KineticLine, KineticSegTreeMin};

pub struct KineticSegTreeMax {
    tree: KineticSegTreeMin,
}

impl KineticSegTreeMax {
    pub fn new(n: usize, f: impl Fn(usize) -> (i64, i64), time: i64) -> Self {
        Self {
            tree: KineticSegTreeMin::new(
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

    pub fn get_max(&mut self, range: Range<usize>) -> KineticLine {
        let line = self.tree.get_min(range);
        KineticLine {
            a: -line.a,
            b: -line.b,
            pos: line.pos,
        }
    }

    pub fn update_time(&mut self, new_time: i64) {
        self.tree.update_time(new_time);
    }
}
