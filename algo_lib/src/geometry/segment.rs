use std::cmp::Ordering;

use crate::geometry::direction::DirectionT;
use crate::geometry::line::Line;
use crate::geometry::point::PointT;
use crate::geometry::segment_intersection::inside_bounding_box;
use crate::misc::num_traits::{HasConstants, Number};
use crate::misc::ord_f64::OrdF64;

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

impl SegmentT<OrdF64> {
    pub fn to_line(&self) -> Line {
        Line::new(&self.from, &self.to)
    }

    pub fn contains(&self, p: &PointT<OrdF64>) -> bool {
        // TODO: should use eps?
        inside_bounding_box(self, p) && PointT::vect_mul(&self.from, &self.to, p) == OrdF64::ZERO
    }

    pub fn len2(&self) -> OrdF64 {
        self.from.dist2(&self.to)
    }

    pub fn len(&self) -> OrdF64 {
        self.len2().sqrt()
    }
}
