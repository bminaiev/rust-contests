use std::cmp::{max, min};

use crate::{
    geometry::point::PointT,
    misc::{min_max::UpdateMinMax, num_traits::Number},
};

#[derive(Clone, Copy)]
pub struct BoundingBox<T: Number> {
    pub min: PointT<T>,
    pub max: PointT<T>,
}

impl<T: Number> BoundingBox<T> {
    pub fn new(first: &PointT<T>, second: &PointT<T>) -> Self {
        let bottom_left = PointT::new(min(first.x, second.x), min(first.y, second.y));
        let top_right = PointT::new(max(first.x, second.x), max(first.y, second.y));
        Self {
            min: bottom_left,
            max: top_right,
        }
    }

    pub fn add(&mut self, p: &PointT<T>) {
        self.min.x.update_min(p.x);
        self.min.y.update_min(p.y);
        self.max.x.update_max(p.x);
        self.max.y.update_max(p.y);
    }

    pub fn dx(&self) -> T {
        self.max.x - self.min.x
    }

    pub fn dy(&self) -> T {
        self.max.y - self.min.y
    }

    pub fn contains(&self, p: &PointT<T>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn to_the_left_of(&self, p1: &PointT<T>, p2: &PointT<T>) -> bool {
        if PointT::vect_mul(p1, p2, &PointT::new(self.min.x, self.min.y)) <= T::ZERO {
            return false;
        }
        if PointT::vect_mul(p1, p2, &PointT::new(self.min.x, self.max.y)) <= T::ZERO {
            return false;
        }
        if PointT::vect_mul(p1, p2, &PointT::new(self.max.x, self.min.y)) <= T::ZERO {
            return false;
        }
        if PointT::vect_mul(p1, p2, &PointT::new(self.max.x, self.max.y)) <= T::ZERO {
            return false;
        }
        true
    }

    // TODO: this is wrong code!!!!???
    pub fn intersect(&self, another: &Self) -> bool {
        if self.to_the_left_of(
            &PointT::new(another.min.x, another.max.y),
            &PointT::new(another.max.x, another.max.y),
        ) {
            return false;
        }

        if self.to_the_left_of(
            &PointT::new(another.max.x, another.max.y),
            &PointT::new(another.max.x, another.min.y),
        ) {
            return false;
        }

        if self.to_the_left_of(
            &PointT::new(another.max.x, another.min.y),
            &PointT::new(another.min.x, another.min.y),
        ) {
            return false;
        }
        if self.to_the_left_of(
            &PointT::new(another.min.x, another.min.y),
            &PointT::new(another.min.x, another.max.y),
        ) {
            return false;
        }

        true
    }
}
