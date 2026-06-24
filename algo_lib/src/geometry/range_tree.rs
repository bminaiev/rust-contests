use std::ops::Range;

use crate::{geometry::point::PointWithIdT, misc::binary_search::binary_search_first_true};

type Point = PointWithIdT<i64>;

#[derive(Clone)]
pub struct RangeTree {
    // sorted by y
    points: Vec<Point>,
    xs: Range<i64>,
    children: Option<(Box<RangeTree>, Box<RangeTree>)>,
}

impl RangeTree {
    pub fn new(mut points: Vec<Point>) -> Self {
        points.sort_by_key(|p| p.p.x);
        let min_x = points[0].p.x;
        let max_x = points[points.len() - 1].p.x + 1;
        let xs = min_x..max_x;
        let mut children = None;
        if min_x + 1 < max_x {
            let mid = points.len().div_ceil(2);
            let left_children = RangeTree::new(points[0..mid].to_vec());
            let right_children = RangeTree::new(points[mid..].to_vec());
            children = Some((Box::new(left_children), Box::new(right_children)));
        }
        points.sort_by_key(|p| p.p.y);
        Self {
            points,
            xs,
            children,
        }
    }

    pub fn iter_rect<F: FnMut(&[Point])>(&self, x: Range<i64>, y: Range<i64>, mut f: F) {
        self.iter_rect_impl(x, y, &mut f);
    }

    fn iter_rect_impl<F: FnMut(&[Point])>(&self, x: Range<i64>, y: Range<i64>, f: &mut F) {
        if self.xs.start >= x.end || self.xs.end <= x.start {
            return;
        }
        if x.start <= self.xs.start && self.xs.end <= x.end {
            let from = binary_search_first_true(0..self.points.len(), |idx| {
                self.points[idx].p.y >= y.start
            });
            let to =
                binary_search_first_true(0..self.points.len(), |idx| self.points[idx].p.y >= y.end);
            f(&self.points[from..to]);
            return;
        }
        if let Some((left, right)) = &self.children {
            left.iter_rect_impl(x.clone(), y.clone(), f);
            right.iter_rect_impl(x, y, f);
        } else {
            unreachable!()
        }
    }
}
