use crate::geometry::point::PointT;

use super::num_traits::Number;

pub struct MatrixIdConverter {
    rows: usize,
    cols: usize,
}

impl MatrixIdConverter {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn get_id_opt<T: Number>(&self, p: &PointT<T>) -> Option<usize> {
        let x = p.x.to_i32();
        let y = p.y.to_i32();
        if x < 0 || y < 0 {
            return None;
        }
        if x >= self.rows as i32 {
            return None;
        }
        if y >= self.cols as i32 {
            return None;
        }
        Some((x as usize) * self.cols + (y as usize))
    }

    pub fn get_id<T: Number>(&self, p: &PointT<T>) -> usize {
        self.get_id_opt(p).unwrap()
    }

    pub fn conv_back(&self, id: usize) -> PointT<i32> {
        assert!(id < self.rows * self.cols);
        let y = id % self.cols;
        let x = id / self.cols;
        PointT::new(x as i32, y as i32)
    }
}
