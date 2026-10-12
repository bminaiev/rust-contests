use crate::geometry::point::PointT;
use crate::math::gcd::gcd;
use crate::misc::double_double::DoubleDouble;
use std::fmt::Debug;

type Point = PointT<DoubleDouble>;

// a*x + b*y + c = 0
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub a: DoubleDouble,
    pub b: DoubleDouble,
    pub c: DoubleDouble,
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
        // debug_assert!(res.on_line(p1));
        // debug_assert!(res.on_line(p2));
        res
    }

    pub fn new_gcd(p1: (i64, i64), p2: (i64, i64)) -> Self {
        let mut a = p2.1 - p1.1;
        let mut b = p1.0 - p2.0;
        let mut c = -(p1.0 * a + p1.1 * b);
        let mut g = gcd(a, b);
        g = gcd(g, c);
        g = g.abs();
        a /= g;
        b /= g;
        c /= g;
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn norm(&mut self) {
        if self.a > DoubleDouble::EPS {
        } else if self.a < -DoubleDouble::EPS {
            self.a = -self.a;
            self.b = -self.b;
            self.c = -self.c;
        } else if self.b > DoubleDouble::EPS {
        } else if self.b < -DoubleDouble::EPS {
            self.b = -self.b;
            self.c = -self.c;
        }
    }

    pub fn new3(mut a: i64, mut b: i64, mut c: i64) -> Self {
        let mut g = gcd(a, b);
        g = gcd(g, c);
        g = g.abs();
        a /= g;
        b /= g;
        c /= g;
        let res = Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        };
        debug_assert!(res.a != DoubleDouble::ZERO || res.b != DoubleDouble::ZERO);
        res
    }

    pub fn on_line(&self, p: &Point) -> bool {
        (self.a * p.x + self.b * p.y + self.c).eq_with_default_eps(&DoubleDouble::ZERO)
    }

    pub fn intersect(&self, other: &Self) -> Option<Point> {
        let denom = self.b * other.a - other.b * self.a;
        if denom.eq_with_default_eps(&DoubleDouble::ZERO) {
            return None;
        }
        let y_num = other.c * self.a - self.c * other.a;
        let x_num = self.c * other.b - other.c * self.b;
        let res = Point::new(x_num / denom, y_num / denom);
        debug_assert!(
            self.abs_dist(&res)
                .eq_with_eps(&DoubleDouble::ZERO, DoubleDouble::EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            self,
            res,
            self.abs_dist(&res)
        );
        debug_assert!(
            other
                .abs_dist(&res)
                .eq_with_eps(&DoubleDouble::ZERO, DoubleDouble::EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            other,
            res,
            self.abs_dist(&res)
        );
        Some(res)
    }

    pub fn signed_dist(&self, p: &Point) -> DoubleDouble {
        (self.a * p.x + self.b * p.y + self.c) / (self.a * self.a + self.b * self.b).sqrt()
    }

    pub fn abs_dist(&self, p: &Point) -> DoubleDouble {
        self.signed_dist(p).abs()
    }

    pub fn abs_dist2(&self, p: &Point) -> DoubleDouble {
        let z = self.a * p.x + self.b * p.y + self.c;
        z * z / (self.a * self.a + self.b * self.b)
    }

    pub fn closest_to_zero(&self) -> Point {
        let den = self.a * self.a + self.b * self.b;
        Point::new(-self.a * self.c / den, -self.b * self.c / den)
    }

    pub fn closest_on_line(&self, p: Point) -> Point {
        let den = self.a * self.a + self.b * self.b;
        let num = self.a * p.x + self.b * p.y + self.c;
        Point::new(p.x - self.a * num / den, p.y - self.b * num / den)
    }

    pub fn mirror(&self, p: Point) -> Point {
        let closest = self.closest_on_line(p);
        Point::new(
            DoubleDouble::TWO * closest.x - p.x,
            DoubleDouble::TWO * closest.y - p.y,
        )
    }

    pub fn project(&self, p: Point) -> DoubleDouble {
        (-self.b * p.x + self.a * p.y) / (self.a * self.a + self.b * self.b).sqrt()
    }
}
