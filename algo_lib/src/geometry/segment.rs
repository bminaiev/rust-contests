use crate::geometry::direction::DirectionT;
use crate::geometry::line::Line;
use crate::geometry::point::PointT;
use crate::misc::num_traits::Number;
use crate::misc::ord_f64::OrdF64;

#[derive(Copy, Clone, Debug)]
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

    pub fn dir(&self) -> DirectionT<T> {
        DirectionT::new(self.from, self.to)
    }

    ///
    /// 1 means "[p] is to the left from [self.from] -> [self.to] ray"
    /// 0 means "on the same line"
    ///
    pub fn to_the_left(&self, p: &PointT<T>) -> i32 {
        let v_mul = PointT::<T>::vect_mul(&self.from, &self.to, p);
        if v_mul > T::ZERO {
            1
        } else if v_mul < T::ZERO {
            -1
        } else {
            0
        }
    }
}

impl SegmentT<OrdF64> {
    pub fn to_line(&self) -> Line {
        Line::new(&self.from, &self.to)
    }
}
