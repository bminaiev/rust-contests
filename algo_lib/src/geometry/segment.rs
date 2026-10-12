use std::cmp::Ordering;

use crate::geometry::direction::DirectionT;
use crate::geometry::line::LineT;
use crate::geometry::point::PointT;
use crate::geometry::segment_intersection::inside_bounding_box;
use crate::misc::num_traits::{Double, Number};

#[derive(Copy, Clone, Debug)]
pub struct SegmentT<T>
where
    T: Number,
{
    pub from: PointT<T>,
    pub to: PointT<T>,
}

impl<T> SegmentT<T>
where
    T: Number + Ord,
{
    pub fn new(from: PointT<T>, to: PointT<T>) -> Self {
        Self { from, to }
    }

    pub fn dir(&self) -> DirectionT<T> {
        DirectionT::new(self.from, self.to)
    }

    ///
    /// 1 means "[p] is to the left from [self.from] -> [self.to] ray"
    /// 0 means "on the same line"
    ///
    pub fn to_the_left(&self, p: &PointT<T>) -> i32 {
        let v_mul = PointT::<T>::vect_mul(&self.from, &self.to, p);
        match v_mul.cmp(&T::ZERO) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

impl<T: Double> SegmentT<T> {
    pub fn to_line(&self) -> LineT<T> {
        LineT::new(&self.from, &self.to)
    }

    /// Uses `T::EPS` for both bounding-box and collinearity checks.
    pub fn contains(&self, p: &PointT<T>) -> bool {
        inside_bounding_box(self, p)
            && PointT::vect_mul(&self.from, &self.to, p).eq_with_default_eps(&T::ZERO)
    }

    pub fn len2(&self) -> T {
        self.from.dist2(&self.to)
    }

    pub fn len(&self) -> T {
        self.len2().sqrt()
    }
}
