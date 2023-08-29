use crate::misc::num_traits::{ConvSimple, HasConstants, Number};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub struct NumberPair<T1: Number, T2: Number> {
    pub first: T1,
    pub second: T2,
}

impl<T1: Number, T2: Number> NumberPair<T1, T2> {
    pub fn new(first: T1, second: T2) -> Self {
        Self { first, second }
    }

    pub fn to_tuple(&self) -> (T1, T2) {
        (self.first, self.second)
    }
}

impl<T1: Number, T2: Number> Copy for NumberPair<T1, T2> {}

impl<T1: Number, T2: Number> Clone for NumberPair<T1, T2> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T1, T2> Add for NumberPair<T1, T2>
where
    T1: Number,
    T2: Number,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.first + rhs.first, self.second + rhs.second)
    }
}

impl<T1: Number, T2: Number> AddAssign for NumberPair<T1, T2> {
    fn add_assign(&mut self, rhs: Self) {
        self.first += rhs.first;
        self.second += rhs.second;
    }
}

impl<T1: Number, T2: Number> Sub for NumberPair<T1, T2> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.first - rhs.first, self.second - rhs.second)
    }
}

impl<T1: Number, T2: Number> SubAssign for NumberPair<T1, T2> {
    fn sub_assign(&mut self, rhs: Self) {
        self.first -= rhs.first;
        self.second -= rhs.second;
    }
}

impl<T1: Number, T2: Number> Mul for NumberPair<T1, T2> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.first * rhs.first, self.second * rhs.second)
    }
}

impl<T1: Number, T2: Number> MulAssign for NumberPair<T1, T2> {
    fn mul_assign(&mut self, rhs: Self) {
        self.first *= rhs.first;
        self.second *= rhs.second;
    }
}

impl<T1: Number, T2: Number> Div for NumberPair<T1, T2> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.first / rhs.first, self.second / rhs.second)
    }
}

impl<T1: Number, T2: Number> DivAssign for NumberPair<T1, T2> {
    fn div_assign(&mut self, rhs: Self) {
        self.first /= rhs.first;
        self.second /= rhs.second;
    }
}

impl<T1: Number, T2: Number> PartialEq<Self> for NumberPair<T1, T2> {
    fn eq(&self, other: &Self) -> bool {
        self.to_tuple().eq(&other.to_tuple())
    }
}

impl<T1: Number, T2: Number> PartialOrd<Self> for NumberPair<T1, T2> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_tuple().partial_cmp(&other.to_tuple())
    }
}

impl<T1: Number + Ord, T2: Number + Ord> Ord for NumberPair<T1, T2> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_tuple().cmp(&other.to_tuple())
    }
}

impl<T1: Number, T2: Number> Eq for NumberPair<T1, T2> {}

impl<T1: Number, T2: Number> HasConstants<Self> for NumberPair<T1, T2> {
    const MAX: Self = NumberPair {
        first: T1::MAX,
        second: T2::MAX,
    };
    const MIN: Self = NumberPair {
        first: T1::MIN,
        second: T2::MIN,
    };
    const ZERO: Self = NumberPair {
        first: T1::ZERO,
        second: T2::ZERO,
    };
    const ONE: Self = NumberPair {
        first: T1::ONE,
        second: T2::ONE,
    };
    const TWO: Self = NumberPair {
        first: T1::TWO,
        second: T2::TWO,
    };
}

impl<T1: Number, T2: Number> Default for NumberPair<T1, T2> {
    fn default() -> Self {
        Self::new(T1::default(), T2::default())
    }
}

impl<T1: Number, T2: Number> Debug for NumberPair<T1, T2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.first.fmt(f)?;
        f.write_str(", ")?;
        self.second.fmt(f)?;
        f.write_str(")")
    }
}

impl<T1: Number, T2: Number> ConvSimple<Self> for NumberPair<T1, T2> {
    fn from_i32(val: i32) -> Self {
        Self::new(T1::from_i32(val), T2::from_i32(val))
    }

    fn to_i32(self) -> i32 {
        panic!("Can't convert pair to i32");
    }

    fn to_f64(self) -> f64 {
        panic!("Can't convert tuple to f64");
    }
}

pub fn number_pair<T1: Number, T2: Number>(first: T1, second: T2) -> NumberPair<T1, T2> {
    NumberPair::new(first, second)
}
