use std::ops::{Index, IndexMut};

pub struct Array2D<T> {
    m: usize,
    v: Vec<T>,
}

impl<T> Array2D<T>
where
    T: Clone,
{
    #[allow(unused)]
    pub fn new(empty: T, n: usize, m: usize) -> Self {
        Self {
            m,
            v: vec![empty; n * m],
        }
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[(index) * self.m..(index + 1) * self.m]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(index) * self.m..(index + 1) * self.m]
    }
}
