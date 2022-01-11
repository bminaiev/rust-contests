use crate::geometry::line::Line;
use crate::geometry::point::PointT;
use crate::misc::num_traits::Number;
use crate::misc::ord_f64::OrdF64;

pub struct SegmentT<T>
where
    T: Number,
{
    pub from: PointT<T>,
    pub to: PointT<T>,
}

impl<T> SegmentT<T>
where
    T: Number,
{
    pub fn new(from: PointT<T>, to: PointT<T>) -> Self {
        Self { from, to }
    }
}

impl SegmentT<OrdF64> {
    pub fn to_line(&self) -> Line {
        Line::new(&self.from, &self.to)
    }
}
