use crate::geometry::line::Line;
use crate::geometry::point::PointT;
use crate::geometry::segment::SegmentT;
use crate::io::input::{Input, Readable};
use crate::misc::num_traits::{HasConstants, Number, Signum};
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

    pub fn new_rect(start: PointT<T>, end: PointT<T>) -> Self {
        Self::new(vec![
            start,
            PointT::new(end.x, start.y),
            end,
            PointT::new(start.x, end.y),
        ])
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

    // To the left of [from] --> [to]
    pub fn cut(&self, from: PointT<T>, to: PointT<T>) -> PolygonT<OrdF64>
    where
        f64: From<T>,
    {
        let l1 = Line::new(&from.conv_float(), &to.conv_float());

        let mut pts = vec![];
        for s in self.edges() {
            let (cur, next) = (s.from, s.to);

            let v_cur = PointT::vect_mul(&from, &to, &cur);
            let v_next = PointT::vect_mul(&from, &to, &next);
            if v_cur >= T::ZERO {
                pts.push(cur.conv_float());
            }
            if v_cur != T::ZERO && v_next != T::ZERO && v_cur.signum() != v_next.signum() {
                let l2 = Line::new(&cur.conv_float(), &next.conv_float());
                let intersection = l1.intersect(&l2).unwrap();
                pts.push(intersection);
            }
        }
        PolygonT::new(pts)
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
