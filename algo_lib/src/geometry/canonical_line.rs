use crate::{geometry::point::PointT, math::gcd::gcd};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalLine {
    a: i64,
    b: i64,
    c: i64,
}

impl CanonicalLine {
    pub fn new(p1: &PointT<i64>, p2: &PointT<i64>) -> Self {
        let a = p2.y - p1.y;
        let b = p1.x - p2.x;
        let mut res = Self {
            a,
            b,
            c: -(p1.x * a + p1.y * b),
        };
        let g = gcd(res.a, gcd(res.b, res.c));
        res.a /= g;
        res.b /= g;
        res.c /= g;
        if (res.a, res.b, res.c) < (0, 0, 0) {
            res.a = -res.a;
            res.b = -res.b;
            res.c = -res.c;
        }
        res
    }
}
