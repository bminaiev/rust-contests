use crate::geometry::point::PointT;
use crate::misc::num_traits::Number;
use std::cmp::Ordering;

///
/// Sorted counter-clock-wise
/// starting from (0;0) -> (inf; 0)
///
pub struct DirectionT<T>(PointT<T>)
where
    T: Number;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Side {
    PositiveY,
    NegativeY,
}

impl<T> DirectionT<T>
where
    T: Number,
{
    pub fn new(from: PointT<T>, to: PointT<T>) -> Self {
        Self(to - from)
    }

    pub fn inverse(&self) -> Self {
        Self(PointT::ZERO - self.0)
    }

    fn side(&self) -> Side {
        if self.0.y > T::ZERO || (self.0.y == T::ZERO && self.0.x >= T::ZERO) {
            Side::PositiveY
        } else {
            Side::NegativeY
        }
    }
}

impl<T> PartialEq<Self> for DirectionT<T>
where
    T: Number + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for DirectionT<T> where T: Number + Ord {}

impl<T> PartialOrd<Self> for DirectionT<T>
where
    T: Number + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.side().cmp(&other.side()).then(
                PointT::<T>::vect_mul2(&self.0, &other.0)
                    .cmp(&T::ZERO)
                    .reverse(),
            ),
        )
    }
}

impl<T> Ord for DirectionT<T>
where
    T: Number + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
