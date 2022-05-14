use std::ops::{Index, Range};

pub struct Permutation {
    ids: Vec<usize>,
    pos_of_element: Vec<usize>,
}

impl Permutation {
    pub fn new(n: usize) -> Self {
        Self::from_vec((0..n).collect())
    }

    pub fn from_vec(ids: Vec<usize>) -> Self {
        let n = ids.len();
        let mut pos_of_element = vec![0; n];
        for (pos, &val) in ids.iter().enumerate() {
            pos_of_element[val] = pos;
        }
        Self {
            ids,
            pos_of_element,
        }
    }

    pub fn get_pos_of_element(&self, value: usize) -> usize {
        let res = self.pos_of_element[value];
        debug_assert_eq!(self.ids[res], value);
        res
    }

    pub fn swap(&mut self, p1: usize, p2: usize) {
        self.ids.swap(p1, p2);
        self.pos_of_element[self.ids[p1]] = p1;
        self.pos_of_element[self.ids[p2]] = p2;
    }

    fn reverse(&mut self, r: Range<usize>) {
        let mut start = r.start;
        let mut end = r.end;
        while start < end {
            end -= 1;
            self.swap(start, end);
            start += 1;
        }
    }

    pub fn next(&mut self) -> bool {
        for pos in (1..(self.ids.len())).rev() {
            if self.ids[pos - 1] < self.ids[pos] {
                for pos2 in (pos..self.ids.len()).rev() {
                    if self.ids[pos - 1] < self.ids[pos2] {
                        self.swap(pos - 1, pos2);
                        self.reverse(pos..self.ids.len());
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
