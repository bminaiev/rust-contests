use crate::misc::num_traits::{ConvSimple, HasConstants};
use std::cmp::Ordering;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A normalized sum of two `f64`s, providing roughly 106 significant bits.
///
/// Arithmetic retains the low component; converting to `f64` rounds back to
/// ordinary double precision. The exponent range is still that of `f64`, and
/// the extra precision is not guaranteed near overflow or underflow.
/// Comparisons follow floating-point semantics: NaN is unordered.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DoubleDouble {
    hi: f64,
    lo: f64,
}

impl DoubleDouble {
    pub const ZERO: Self = Self { hi: 0.0, lo: 0.0 };
    pub const ONE: Self = Self { hi: 1.0, lo: 0.0 };
    pub const TWO: Self = Self { hi: 2.0, lo: 0.0 };
    pub const NAN: Self = Self {
        hi: f64::NAN,
        lo: 0.0,
    };
    pub const INFINITY: Self = Self {
        hi: f64::INFINITY,
        lo: 0.0,
    };
    pub const NEG_INFINITY: Self = Self {
        hi: f64::NEG_INFINITY,
        lo: 0.0,
    };
    pub const MAX: Self = Self {
        hi: f64::MAX,
        // Largest f64 below half an ulp of f64::MAX.
        lo: 9.979201547673598e291,
    };
    pub const MIN: Self = Self {
        hi: -Self::MAX.hi,
        lo: -Self::MAX.lo,
    };

    /// Normalizes `hi + lo` using error-free TwoSum, without requiring either
    /// input to have the larger magnitude. The fields stay private so that
    /// arithmetic and comparisons can rely on this normalization.
    pub fn new(hi: f64, lo: f64) -> Self {
        let sum = hi + lo;
        if !sum.is_finite() {
            return Self::from(sum);
        }
        let v = sum - hi;
        let error = (hi - (sum - v)) + (lo - v);
        Self { hi: sum, lo: error }
    }

    pub fn to_f64(self) -> f64 {
        self.hi + self.lo
    }

    /// Truncates toward zero, saturating at the integer bounds (NaN becomes 0).
    /// Unlike casting `to_f64()`, this preserves integer bits in the low part.
    pub fn to_i128(self) -> i128 {
        if !self.hi.is_finite() {
            return self.hi as i128;
        }
        let limit = -(i128::MIN as f64);
        if self.hi >= limit {
            if self.hi > limit || self.lo >= 0.0 {
                return i128::MAX;
            }
            return (i128::MAX + self.lo.floor() as i128) + 1;
        }
        if self.hi <= -limit {
            if self.hi < -limit || self.lo <= 0.0 {
                return i128::MIN;
            }
            return i128::MIN + self.lo.ceil() as i128;
        }
        let hi = self.hi as i128;
        let lo = self.lo as i128;
        let integer = hi + lo;
        let fraction = (self.hi - hi as f64) + (self.lo - lo as f64);
        if integer > 0 && fraction < 0.0 {
            integer - 1
        } else if integer < 0 && fraction > 0.0 {
            integer + 1
        } else {
            integer
        }
    }

    pub fn abs(self) -> Self {
        if self.hi.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    pub fn is_finite(self) -> bool {
        self.hi.is_finite()
    }

    pub fn is_nan(self) -> bool {
        self.hi.is_nan()
    }
}

impl Add for DoubleDouble {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let high = Self::new(self.hi, rhs.hi);
        if !high.is_finite() {
            return high;
        }
        let low = Self::new(self.lo, rhs.lo);
        let sum = Self::new(high.hi, high.lo + low.hi);
        Self::new(sum.hi, sum.lo + low.lo)
    }
}

impl Sub for DoubleDouble {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Mul for DoubleDouble {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let hi = self.hi * rhs.hi;
        if !hi.is_finite() {
            return Self::from(hi);
        }
        let lo = self.hi.mul_add(rhs.hi, -hi);
        let lo = self.hi.mul_add(rhs.lo, lo);
        let lo = self.lo.mul_add(rhs.hi, lo);
        let lo = self.lo.mul_add(rhs.lo, lo);
        Self::new(hi, lo)
    }
}

impl Div for DoubleDouble {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let q1 = Self::from(self.hi / rhs.hi);
        if !q1.is_finite() || !rhs.is_finite() || self.hi == 0.0 {
            return q1;
        }
        let r1 = self - rhs * q1;
        let q2 = Self::from(r1.hi / rhs.hi);
        let r2 = r1 - rhs * q2;
        let q3 = Self::from(r2.hi / rhs.hi);
        q1 + q2 + q3
    }
}

impl Neg for DoubleDouble {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
        }
    }
}

impl AddAssign for DoubleDouble {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for DoubleDouble {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for DoubleDouble {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for DoubleDouble {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl PartialOrd for DoubleDouble {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hi.partial_cmp(&other.hi)? {
            Ordering::Equal => self.lo.partial_cmp(&other.lo),
            order => Some(order),
        }
    }
}

impl From<f64> for DoubleDouble {
    fn from(value: f64) -> Self {
        Self { hi: value, lo: 0.0 }
    }
}

impl From<i128> for DoubleDouble {
    fn from(value: i128) -> Self {
        let hi = value as f64;
        // A positive value may round to 2^127, outside the i128 range.
        let remainder = if hi == -(i128::MIN as f64) {
            value - i128::MAX - 1
        } else {
            value - hi as i128
        };
        Self::new(hi, remainder as f64)
    }
}

impl From<u128> for DoubleDouble {
    fn from(value: u128) -> Self {
        let high = Self::from((value >> 64) as i128);
        let scale = 18446744073709551616.0; // 2^64, exact in f64.
        Self {
            hi: high.hi * scale,
            lo: high.lo * scale,
        } + Self::from(value as u64)
    }
}

macro_rules! from_integer {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for DoubleDouble {
            fn from(value: $ty) -> Self {
                Self::from(value as i128)
            }
        })*
    };
}

from_integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl From<f32> for DoubleDouble {
    fn from(value: f32) -> Self {
        Self::from(value as f64)
    }
}

impl From<DoubleDouble> for f64 {
    fn from(value: DoubleDouble) -> Self {
        value.to_f64()
    }
}

impl HasConstants<Self> for DoubleDouble {
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    const TWO: Self = Self::TWO;
}

impl ConvSimple<Self> for DoubleDouble {
    fn from_i32(value: i32) -> Self {
        Self::from(value)
    }

    fn to_i32(self) -> i32 {
        self.to_i128().clamp(i32::MIN as i128, i32::MAX as i128) as i32
    }

    fn to_f64(self) -> f64 {
        self.to_f64()
    }

    fn to_i128(self) -> i128 {
        self.to_i128()
    }
}

impl Sum for DoubleDouble {
    /// Pairwise summation, retaining O(log n) partial sums of power-of-two sizes.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut partials: Vec<Option<Self>> = Vec::new();
        for mut value in iter {
            let mut level = 0;
            loop {
                if level == partials.len() {
                    partials.push(Some(value));
                    break;
                }
                if let Some(previous) = partials[level].take() {
                    value = previous + value;
                    level += 1;
                } else {
                    partials[level] = Some(value);
                    break;
                }
            }
        }
        partials
            .into_iter()
            .flatten()
            .fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for DoubleDouble {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for DoubleDouble {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Self> for DoubleDouble {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}
