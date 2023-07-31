use std::cmp::Ordering;

use crate::{
    math::gcd::gcd,
    misc::num_traits::{ConvSimple, HasConstants, Number},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct FracT<T: Number> {
    pub num: T,
    pub denum: T,
}

impl<T: Number + std::ops::Rem<Output = T>> FracT<T> {
    pub fn new(mut num: T, mut denum: T) -> Self {
        if denum == T::ZERO {
            return match num.cmp(&T::ZERO) {
                Ordering::Less => Self {
                    num: T::ZERO - T::ONE,
                    denum: T::ZERO,
                },
                Ordering::Equal => Self {
                    num: T::ZERO,
                    denum: T::ZERO,
                },
                Ordering::Greater => Self {
                    num: T::ONE,
                    denum: T::ZERO,
                },
            };
        }
        if denum < T::ZERO {
            num *= T::ZERO - T::ONE;
            denum *= T::ZERO - T::ONE;
        }
        let g = gcd(num, denum);
        Self {
            num: num / g,
            denum: denum / g,
        }
    }
}

impl<T: Number> PartialEq for FracT<T> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.denum == other.denum
    }
}
impl<T: Number> Eq for FracT<T> {}

impl<T: Number> PartialOrd for FracT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.num * other.denum).cmp(&(other.num * self.denum)))
    }
}

impl<T: Number> Ord for FracT<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::Mul for FracT<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.denum * rhs.denum)
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::MulAssign for FracT<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::Add for FracT<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.num * rhs.denum + rhs.num * self.denum,
            self.denum * rhs.denum,
        )
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::AddAssign for FracT<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::Div for FracT<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.denum, self.denum * rhs.num)
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::DivAssign for FracT<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::Sub for FracT<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.num * rhs.denum - rhs.num * self.denum,
            self.denum * rhs.denum,
        )
    }
}

impl<T: Number + std::ops::Rem<Output = T>> std::ops::SubAssign for FracT<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: Number + std::ops::Rem<Output = T>> HasConstants<FracT<T>> for FracT<T> {
    const MAX: Self = FracT {
        num: T::MAX,
        denum: T::ONE,
    };

    const MIN: Self = FracT {
        num: T::MIN,
        denum: T::ONE,
    };

    const ZERO: Self = FracT {
        num: T::ZERO,
        denum: T::ONE,
    };

    const ONE: Self = FracT {
        num: T::ONE,
        denum: T::ONE,
    };

    const TWO: Self = FracT {
        num: T::TWO,
        denum: T::ONE,
    };
}

impl<T: Number + std::ops::Rem<Output = T>> ConvSimple<Self> for FracT<T> {
    fn from_i32(val: i32) -> Self {
        Self::new(T::from_i32(val), T::ONE)
    }

    fn to_i32(self) -> i32 {
        (self.num / self.denum).to_i32()
    }

    fn to_f64(self) -> f64 {
        self.num.to_f64() / self.denum.to_f64()
    }
}
