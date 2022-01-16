use std::ops::Index;

pub struct Permutation {
    ids: Vec<usize>,
}

impl Permutation {
    pub fn new(n: usize) -> Self {
        Self {
            ids: (0..n).collect(),
        }
    }

    pub fn next(&mut self) -> bool {
        for pos in (1..(self.ids.len())).rev() {
            if self.ids[pos - 1] < self.ids[pos] {
                for pos2 in (pos..self.ids.len()).rev() {
                    if self.ids[pos - 1] < self.ids[pos2] {
                        self.ids.swap(pos - 1, pos2);
                        self.ids[pos..].reverse();
                        return true;
                    }
                }
                unreachable!();
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }
}

impl Index<usize> for Permutation {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ids[index]
    }
}
