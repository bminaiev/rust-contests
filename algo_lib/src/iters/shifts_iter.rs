use crate::geometry::point::PointT;

use super::shifts::Shift;

pub struct ShiftsIterator {
    shifts: Vec<Shift>,
    rows: usize,
    cols: usize,
}

pub struct Iter<'a> {
    info: &'a ShiftsIterator,
    pos: usize,
    p: PointT<i32>,
}

impl ShiftsIterator {
    pub fn new(shifts: &[Shift], rows: usize, cols: usize) -> Self {
        Self {
            shifts: shifts.to_vec(),
            rows,
            cols,
        }
    }

    pub fn iter(&self, row: usize, col: usize) -> Iter {
        Iter {
            info: self,
            pos: 0,
            p: PointT::new(row as i32, col as i32),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos != self.info.shifts.len() {
            let shift = self.info.shifts[self.pos];
            self.pos += 1;
            let res = self.p.apply_shift(&shift);
            if res.x >= 0
                && res.y >= 0
                && res.x < self.info.rows as i32
                && res.y < self.info.cols as i32
            {
                return Some((res.x as usize, res.y as usize));
            }
        }
        None
    }
}
