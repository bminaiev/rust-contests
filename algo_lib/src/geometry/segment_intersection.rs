use crate::geometry::point::PointT;
use crate::geometry::segment::SegmentT;
use crate::misc::ord_f64::OrdF64;
use std::cmp::{max, min};

type Point = PointT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn inside_one_dim(range: (OrdF64, OrdF64), val: OrdF64) -> bool {
    min(range.0, range.1) - OrdF64::EPS <= val && val <= max(range.0, range.1) + OrdF64::EPS
}

pub fn inside_bounding_box(seg: &Segment, p: &Point) -> bool {
    inside_one_dim((seg.from.x, seg.to.x), p.x) && inside_one_dim((seg.from.y, seg.to.y), p.y)
}

pub fn segment_intersection(seg1: &Segment, seg2: &Segment) -> Option<Point> {
    seg1.to_line()
        .intersect(&seg2.to_line())
        .filter(|&inter| inside_bounding_box(seg1, &inter) && inside_bounding_box(seg2, &inter))
}
