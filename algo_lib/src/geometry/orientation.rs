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

pub fn remove_three_on_line<T: Number>(mut poly: Vec<PointT<T>>) -> Vec<PointT<T>> {
    let first = poly.index_of_min();
    poly.rotate_left(first);
    let mut result = vec![];
    for p in poly.into_iter() {
        if result.len() >= 2 {
            let p1 = result[result.len() - 2];
            let p2 = result[result.len() - 1];
            if PointT::vect_mul(&p1, &p2, &p) == T::ZERO {
                result.pop();
            }
        }
        result.push(p);
    }
    if result.len() >= 3 {
        let p1 = result[result.len() - 2];
        let p2 = result[result.len() - 1];
        let p3 = result[0];
        if PointT::vect_mul(&p1, &p2, &p3) == T::ZERO {
            result.pop();
        }
    }
    result
}
