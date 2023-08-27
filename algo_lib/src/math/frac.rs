use std::cmp::Ordering;

use crate::{
    math::gcd::gcd,
    misc::num_traits::{ConvSimple, HasConstants, Number},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct FracT<T: Number> {
    pub num: T,
    pub denom: T,
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> FracT<T> {
    pub fn new(mut num: T, mut denom: T) -> Self {
        if denom == T::ZERO {
            return match num.cmp(&T::ZERO) {
                Ordering::Less => Self {
                    num: T::ZERO - T::ONE,
                    denom: T::ZERO,
                },
                Ordering::Equal => Self {
                    num: T::ZERO,
                    denom: T::ZERO,
                },
                Ordering::Greater => Self {
                    num: T::ONE,
                    denom: T::ZERO,
                },
            };
        }
        if denom < T::ZERO {
            num *= T::ZERO - T::ONE;
            denom *= T::ZERO - T::ONE;
        }
        let g = gcd(num, denom);
        Self {
            num: num / g,
            denom: denom / g,
        }
    }
}

impl<T: Number> PartialEq for FracT<T> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.denom == other.denom
    }
}
impl<T: Number> Eq for FracT<T> {}

impl<T: Number + Ord> PartialOrd for FracT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.num * other.denom).cmp(&(other.num * self.denom)))
    }
}

impl<T: Number + Ord> Ord for FracT<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Mul for FracT<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.denom * rhs.denom)
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::MulAssign for FracT<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Add for FracT<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.num * rhs.denom + rhs.num * self.denom,
            self.denom * rhs.denom,
        )
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::AddAssign for FracT<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Div for FracT<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.denom, self.denom * rhs.num)
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::DivAssign for FracT<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Sub for FracT<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.num * rhs.denom - rhs.num * self.denom,
            self.denom * rhs.denom,
        )
    }
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::SubAssign for FracT<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T>> HasConstants<FracT<T>> for FracT<T> {
    const MAX: Self = FracT {
        num: T::MAX,
        denom: T::ONE,
    };

    const MIN: Self = FracT {
        num: T::MIN,
        denom: T::ONE,
    };

    const ZERO: Self = FracT {
        num: T::ZERO,
        denom: T::ONE,
    };

    const ONE: Self = FracT {
        num: T::ONE,
        denom: T::ONE,
    };

    const TWO: Self = FracT {
        num: T::TWO,
        denom: T::ONE,
    };
}

impl<T: Number + std::ops::Rem<Output = T> + Ord> ConvSimple<Self> for FracT<T> {
    fn from_i32(val: i32) -> Self {
        Self::new(T::from_i32(val), T::ONE)
    }

    fn to_i32(self) -> i32 {
        (self.num / self.denom).to_i32()
    }

    fn to_f64(self) -> f64 {
        self.num.to_f64() / self.denom.to_f64()
    }
}
