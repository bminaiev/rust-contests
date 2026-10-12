use crate::geometry::segment::SegmentT;
use crate::{geometry::point::PointT, misc::double_double::DoubleDouble};
use std::cmp::{max, min};

type Point = PointT<DoubleDouble>;
type Segment = SegmentT<DoubleDouble>;

fn inside_one_dim(range: (DoubleDouble, DoubleDouble), val: DoubleDouble) -> bool {
    min(range.0, range.1) - DoubleDouble::EPS <= val
        && val <= max(range.0, range.1) + DoubleDouble::EPS
}

pub fn inside_bounding_box_dd(seg: &Segment, p: &Point) -> bool {
    inside_one_dim((seg.from.x, seg.to.x), p.x) && inside_one_dim((seg.from.y, seg.to.y), p.y)
}

pub fn segment_intersection_dd(seg1: &Segment, seg2: &Segment) -> Option<Point> {
    seg1.to_line().intersect(&seg2.to_line()).filter(|&inter| {
        inside_bounding_box_dd(seg1, &inter) && inside_bounding_box_dd(seg2, &inter)
    })
}
