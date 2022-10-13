use crate::{
    geometry::point::PointT,
    misc::{min_max::FindMinMaxPos, num_traits::Number},
};

pub fn make_ccw<T: Number>(mut poly: Vec<PointT<T>>) -> Vec<PointT<T>> {
    let first = poly.index_of_min();
    let cur = poly[first];
    let next = poly[(first + 1) % poly.len()];
    let prev = poly[(first + poly.len() - 1) % poly.len()];
    if PointT::vect_mul(&cur, &next, &prev) < T::ZERO {
        poly.reverse();
    }
    poly
}
