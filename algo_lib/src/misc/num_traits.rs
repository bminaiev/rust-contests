use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait HasConstants<T> {
    const MAX: T;
    const MIN: T;
    const ZERO: T;
    const ONE: T;
    const TWO: T;
}

pub trait ConvSimple<T> {
    fn from_i32(val: i32) -> T;
    fn to_i32(self) -> i32;
    fn to_f64(self) -> f64;
    fn to_i128(self) -> i128;
}

pub trait Signum {
    fn signum(&self) -> i32;
}

pub trait Number:
    Copy
    + Clone
    + Default
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + PartialOrd
    + PartialEq
    + HasConstants<Self>
    + Default
    + Debug
    + Sized
    + ConvSimple<Self>
{
}

/// Ordered floating-point arithmetic used by geometry algorithms.
/// Implementations must retain their own precision in arithmetic and square roots.
/// As with `OrdF64`, ordering assumes operands are not NaN.
pub trait Double: Number + Ord + Neg<Output = Self> + From<f64> + From<i64> {
    const EPS: Self;
    /// Tolerance for checking computed intersections in debug assertions.
    const SMALL_EPS: Self = Self::EPS;

    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn is_finite(self) -> bool;

    /// Accepts either absolute error or relative error against the smaller magnitude.
    /// NaN never compares equal; infinities compare equal only to themselves.
    fn eq_with_eps(&self, other: &Self, eps: Self) -> bool {
        if !self.is_finite() || !other.is_finite() {
            return self == other;
        }
        let abs_diff = (*self - *other).abs();
        abs_diff <= eps || abs_diff <= self.abs().min(other.abs()) * eps
    }

    fn eq_with_default_eps(&self, other: &Self) -> bool {
        self.eq_with_eps(other, Self::EPS)
    }
}

impl<
        T: Copy
            + Add<Output = Self>
            + AddAssign
            + Sub<Output = Self>
            + SubAssign
            + Mul<Output = Self>
            + MulAssign
            + Div<Output = Self>
            + DivAssign
            + PartialOrd
            + PartialEq
            + HasConstants<Self>
            + Default
            + Debug
            + Sized
            + ConvSimple<Self>,
    > Number for T
{
}

macro_rules! has_constants_impl {
    ($t: ident) => {
        impl HasConstants<$t> for $t {
            const MAX: $t = $t::MAX;
            const MIN: $t = $t::MIN;
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TWO: $t = 2;
        }

        impl ConvSimple<$t> for $t {
            fn from_i32(val: i32) -> $t {
                val as $t
            }

            fn to_i32(self) -> i32 {
                self as i32
            }

            fn to_f64(self) -> f64 {
                self as f64
            }

            fn to_i128(self) -> i128 {
                self as i128
            }
        }
    };
}

has_constants_impl!(i32);
has_constants_impl!(i64);
has_constants_impl!(i128);
has_constants_impl!(u32);
has_constants_impl!(u64);
has_constants_impl!(u128);
has_constants_impl!(usize);
has_constants_impl!(u8);

impl ConvSimple<Self> for f64 {
    fn from_i32(val: i32) -> Self {
        val as f64
    }

    fn to_i32(self) -> i32 {
        self as i32
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn to_i128(self) -> i128 {
        self as i128
    }
}

impl HasConstants<Self> for f64 {
    const MAX: Self = Self::MAX;
    const MIN: Self = -Self::MAX;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
}

impl<T: Number + Ord> Signum for T {
    fn signum(&self) -> i32 {
        match self.cmp(&T::ZERO) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        }
    }
}
