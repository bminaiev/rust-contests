use std::ops::{Index, IndexMut};

pub struct Array2D<T> {
    rows: usize,
    cols: usize,
    v: Vec<T>,
}

impl<T> Array2D<T>
where
    T: Clone,
{
    #[allow(unused)]
    pub fn new(empty: T, rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            v: vec![empty; rows * cols],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn swap(&mut self, row1: usize, row2: usize) {
        assert!(row1 < self.rows);
        assert!(row2 < self.rows);
        if row1 != row2 {
            for col in 0..self.cols {
                self.v.swap(row1 * self.cols + col, row2 * self.cols + col);
            }
        }
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[(index) * self.cols..(index + 1) * self.cols]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(index) * self.cols..(index + 1) * self.cols]
    }
}
