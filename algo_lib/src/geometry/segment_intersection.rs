use crate::geometry::line::Line;
use crate::geometry::point::PointT;
use crate::geometry::segment::SegmentT;
use crate::misc::ord_f64::OrdF64;
use std::cmp::{max, min};

type Point = PointT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn inside_one_dim(range: (OrdF64, OrdF64), val: OrdF64) -> bool {
    min(range.0, range.1) - OrdF64::EPS <= val && val <= max(range.0, range.1) + OrdF64::EPS
}

fn inside_bounding_box(seg: &Segment, p: &Point) -> bool {
    inside_one_dim((seg.from.x, seg.to.x), p.x) && inside_one_dim((seg.from.y, seg.to.y), p.y)
}

pub fn segment_intersection(seg1: &Segment, seg2: &Segment) -> Option<Point> {
    let line1 = Line::new(&seg1.from, &seg1.to);
    let line2 = Line::new(&seg2.from, &seg2.to);
    if let Some(inter) = line1.intersect(&line2) {
        if inside_bounding_box(seg1, &inter) && inside_bounding_box(seg2, &inter) {
            Some(inter)
        } else {
            None
        }
    } else {
        None
    }
}
