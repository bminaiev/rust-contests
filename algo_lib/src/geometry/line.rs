use crate::geometry::point::PointT;
use crate::misc::num_traits::HasConstants;
use crate::misc::ord_f64::OrdF64;
use std::fmt::Debug;

type Point = PointT<OrdF64>;

// a*x + b*y + c = 0
#[derive(Debug)]
pub struct Line {
    a: OrdF64,
    b: OrdF64,
    c: OrdF64,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let a = p2.y - p1.y;
        let b = p1.x - p2.x;
        let res = Self {
            a,
            b,
            c: -(p1.x * a + p1.y * b),
        };
        debug_assert!(res.on_line(p1));
        debug_assert!(res.on_line(p2));
        res
    }

    pub fn on_line(&self, p: &Point) -> bool {
        (self.a * p.x + self.b * p.y + self.c).eq_with_default_eps(&OrdF64::ZERO)
    }

    pub fn intersect(&self, other: &Self) -> Option<Point> {
        let denum = self.b * other.a - other.b * self.a;
        if denum.eq_with_default_eps(&OrdF64::ZERO) {
            return None;
        }
        let y_num = other.c * self.a - self.c * other.a;
        let x_num = self.c * other.b - other.c * self.b;
        let res = Point::new(x_num / denum, y_num / denum);
        debug_assert!(
            self.abs_dist(&res)
                .eq_with_eps(&OrdF64::ZERO, OrdF64::SMALL_EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            self,
            res,
            self.abs_dist(&res)
        );
        debug_assert!(
            other
                .abs_dist(&res)
                .eq_with_eps(&OrdF64::ZERO, OrdF64::SMALL_EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            other,
            res,
            self.abs_dist(&res)
        );
        Some(res)
    }

    pub fn signed_dist(&self, p: &Point) -> OrdF64 {
        (self.a * p.x + self.b * p.y + self.c) / (self.a * self.a + self.b * self.b).sqrt()
    }

    pub fn abs_dist(&self, p: &Point) -> OrdF64 {
        self.signed_dist(p).abs()
    }

    pub fn abs_dist2(&self, p: &Point) -> OrdF64 {
        let z = self.a * p.x + self.b * p.y + self.c;
        z * z / (self.a * self.a + self.b * self.b)
    }

    pub fn closest_to_zero(&self) -> Point {
        let den = self.a * self.a + self.b * self.b;
        Point::new(-self.a * self.c / den, -self.b * self.c / den)
    }
}
