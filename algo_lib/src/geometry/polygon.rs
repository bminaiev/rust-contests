use crate::geometry::point::PointT;
use crate::geometry::segment::SegmentT;
use crate::io::input::{Input, Readable};
use crate::misc::num_traits::{HasConstants, Number};
use crate::misc::ord_f64::OrdF64;
use std::fmt::{Debug, Formatter};

pub struct PolygonT<T>
where
    T: Number,
{
    points: Vec<PointT<T>>,
}

pub struct PolygonEdgeIter<'a, T>
where
    T: Number,
{
    polygon: &'a PolygonT<T>,
    pos: u32,
}

impl<T> PolygonT<T>
where
    T: Number,
{
    pub fn new(mut points: Vec<PointT<T>>) -> Self {
        assert_ne!(points.len(), 0);
        points.push(points[0]);
        Self { points }
    }

    pub fn points(&self) -> &[PointT<T>] {
        &self.points[0..self.points.len() - 1]
    }

    pub fn edges(&self) -> PolygonEdgeIter<T> {
        PolygonEdgeIter {
            polygon: self,
            pos: 0,
        }
    }

    pub fn min_x(&self) -> T {
        self.points.iter().map(|p| p.x).min().unwrap()
    }

    pub fn max_x(&self) -> T {
        self.points.iter().map(|p| p.x).max().unwrap()
    }

    pub fn min_y(&self) -> T {
        self.points.iter().map(|p| p.y).min().unwrap()
    }

    pub fn max_y(&self) -> T {
        self.points.iter().map(|p| p.y).max().unwrap()
    }

    pub fn area_x2(&self) -> T {
        let mut res = T::ZERO;
        for edge in self.edges() {
            res += edge.from.x * edge.to.y - edge.to.x * edge.from.y;
        }
        if res < T::ZERO {
            T::ZERO - res
        } else {
            res
        }
    }
}

impl<'a, T> Iterator for PolygonEdgeIter<'a, T>
where
    T: Number,
{
    type Item = SegmentT<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.polygon.points[self.pos as usize];
        self.pos += 1;
        let second = self.polygon.points.get(self.pos as usize);
        if let Some(second) = second {
            Some(SegmentT::new(first, *second))
        } else {
            None
        }
    }
}

impl<T> Readable for PolygonT<T>
where
    T: Number + Readable,
{
    fn read(input: &mut Input) -> Self {
        let n = input.usize();
        Self::new(input.vec::<PointT<T>>(n))
    }
}

impl<T> Debug for PolygonT<T>
where
    T: Number,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for (id, p) in self.points().iter().enumerate() {
            writeln!(f, " {}:: ({:?}; {:?})", id, p.x, p.y)?;
        }
        writeln!(f, "]")
    }
}

impl PolygonT<OrdF64> {
    pub fn area(&self) -> OrdF64 {
        self.area_x2() / OrdF64::TWO
    }
}
