use crate::geometry::point::PointT;
use crate::geometry::segment::SegmentT;
use crate::misc::num_traits::Double;
use std::cmp::{max, min};

fn inside_one_dim<T: Double>(range: (T, T), val: T) -> bool {
    min(range.0, range.1) - T::EPS <= val && val <= max(range.0, range.1) + T::EPS
}

pub fn inside_bounding_box<T: Double>(seg: &SegmentT<T>, p: &PointT<T>) -> bool {
    inside_one_dim((seg.from.x, seg.to.x), p.x) && inside_one_dim((seg.from.y, seg.to.y), p.y)
}

pub fn segment_intersection<T: Double>(
    seg1: &SegmentT<T>,
    seg2: &SegmentT<T>,
) -> Option<PointT<T>> {
    seg1.to_line()
        .intersect(&seg2.to_line())
        .filter(|&inter| inside_bounding_box(seg1, &inter) && inside_bounding_box(seg2, &inter))
}
