use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub const BASE: i64 = 1_000_000_000;

#[derive(Clone, Eq, PartialEq)]
pub struct BigInt {
    sign: i8,
    digits: Vec<i64>,
}

impl Default for BigInt {
    fn default() -> Self {
        Self {
            sign: 1,
            digits: Vec::new(),
        }
    }
}

#[doc(hidden)]
pub trait BigIntValue {
    fn into_big_int(self) -> BigInt;
}

macro_rules! big_int_value_signed {
    ($($t:ty),+) => {
        $(
            impl BigIntValue for $t {
                fn into_big_int(self) -> BigInt {
                    BigInt::from_i128(self as i128)
                }
            }
        )+
    };
}

macro_rules! big_int_value_unsigned {
    ($($t:ty),+) => {
        $(
            impl BigIntValue for $t {
                fn into_big_int(self) -> BigInt {
                    BigInt::from_u128(self as u128)
                }
            }
        )+
    };
}

big_int_value_signed!(i8, i16, i32, i64, i128, isize);
big_int_value_unsigned!(u8, u16, u32, u64, u128, usize);

impl<T: BigIntValue> From<T> for BigInt {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl BigInt {
    pub fn new<T: BigIntValue>(value: T) -> Self {
        value.into_big_int()
    }

    pub fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn abs(&self) -> Self {
        let mut res = self.clone();
        res.sign = 1;
        res
    }

    pub fn div_mod(&self, rhs: &Self) -> (Self, Self) {
        assert!(!rhs.is_zero(), "division by zero");
        let (mut q, mut r) = Self::div_mod_abs(&self.abs(), &rhs.abs());
        if !q.is_zero() {
            q.sign = self.sign * rhs.sign;
        }
        if !r.is_zero() {
            r.sign = self.sign;
        }
        (q, r)
    }

    fn from_u128(mut value: u128) -> Self {
        if value == 0 {
            return Self::default();
        }
        let mut digits = Vec::new();
        while value > 0 {
            digits.push((value % BASE as u128) as i64);
            value /= BASE as u128;
        }
        Self { sign: 1, digits }
    }

    fn from_i128(value: i128) -> Self {
        if value >= 0 {
            Self::from_u128(value as u128)
        } else {
            let mut res = Self::from_u128(value.unsigned_abs());
            res.sign = -1;
            res
        }
    }

    fn with_digits(sign: i8, digits: Vec<i64>) -> Self {
        let mut res = Self { sign, digits };
        res.normalize();
        res
    }

    fn normalize(&mut self) {
        while self.digits.last() == Some(&0) {
            self.digits.pop();
        }
        if self.digits.is_empty() {
            self.sign = 1;
        }
    }

    fn cmp_abs(&self, rhs: &Self) -> Ordering {
        match self.digits.len().cmp(&rhs.digits.len()) {
            Ordering::Equal => {}
            ord => return ord,
        }
        for i in (0..self.digits.len()).rev() {
            match self.digits[i].cmp(&rhs.digits[i]) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }
        Ordering::Equal
    }

    fn add_abs(lhs: &[i64], rhs: &[i64]) -> Vec<i64> {
        let n = lhs.len().max(rhs.len());
        let mut res = Vec::with_capacity(n + 1);
        let mut carry = 0;
        for i in 0..n {
            let mut cur = carry;
            if i < lhs.len() {
                cur += lhs[i];
            }
            if i < rhs.len() {
                cur += rhs[i];
            }
            res.push(cur % BASE);
            carry = cur / BASE;
        }
        if carry != 0 {
            res.push(carry);
        }
        res
    }

    fn sub_abs(lhs: &[i64], rhs: &[i64]) -> Vec<i64> {
        let mut res = Vec::with_capacity(lhs.len());
        let mut carry = 0;
        for i in 0..lhs.len() {
            let mut cur = lhs[i] - carry;
            if i < rhs.len() {
                cur -= rhs[i];
            }
            if cur < 0 {
                cur += BASE;
                carry = 1;
            } else {
                carry = 0;
            }
            res.push(cur);
        }
        res
    }

    fn mul_small(&self, rhs: i64) -> Self {
        if rhs == 0 || self.is_zero() {
            return Self::default();
        }
        let mut res = Vec::with_capacity(self.digits.len() + 1);
        let mut carry = 0;
        for &digit in &self.digits {
            let cur = digit * rhs + carry;
            res.push(cur % BASE);
            carry = cur / BASE;
        }
        if carry != 0 {
            res.push(carry);
        }
        Self {
            sign: 1,
            digits: res,
        }
    }

    fn shift_base_add(&mut self, digit: i64) {
        if self.is_zero() {
            if digit != 0 {
                self.digits.push(digit);
            }
        } else {
            self.digits.insert(0, digit);
        }
    }

    fn div_mod_abs(lhs: &Self, rhs: &Self) -> (Self, Self) {
        if lhs.cmp_abs(rhs) == Ordering::Less {
            return (Self::default(), lhs.clone());
        }
        let mut q = vec![0; lhs.digits.len()];
        let mut cur = Self::default();
        for i in (0..lhs.digits.len()).rev() {
            cur.shift_base_add(lhs.digits[i]);
            let mut low = 0;
            let mut high = BASE - 1;
            let mut best = 0;
            while low <= high {
                let mid = (low + high) / 2;
                if rhs.mul_small(mid).cmp_abs(&cur) != Ordering::Greater {
                    best = mid;
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
            q[i] = best;
            if best != 0 {
                cur = cur - rhs.mul_small(best);
            }
        }
        (Self::with_digits(1, q), cur)
    }
}

impl Ord for BigInt {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.sign.cmp(&rhs.sign) {
            Ordering::Equal => {}
            ord => return ord,
        }
        if self.sign > 0 {
            self.cmp_abs(rhs)
        } else {
            rhs.cmp_abs(self)
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        if self.sign < 0 {
            write!(f, "-")?;
        }
        write!(f, "{}", self.digits[self.digits.len() - 1])?;
        for digit in self.digits[..self.digits.len() - 1].iter().rev() {
            write!(f, "{digit:09}")?;
        }
        Ok(())
    }
}

impl Debug for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Neg for BigInt {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if !self.is_zero() {
            self.sign *= -1;
        }
        self
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.sign == rhs.sign {
            Self::with_digits(self.sign, Self::add_abs(&self.digits, &rhs.digits))
        } else {
            match self.cmp_abs(&rhs) {
                Ordering::Less => {
                    Self::with_digits(rhs.sign, Self::sub_abs(&rhs.digits, &self.digits))
                }
                Ordering::Equal => Self::default(),
                Ordering::Greater => {
                    Self::with_digits(self.sign, Self::sub_abs(&self.digits, &rhs.digits))
                }
            }
        }
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return Self::default();
        }
        let mut res = vec![0; self.digits.len() + rhs.digits.len()];
        for i in 0..self.digits.len() {
            let mut carry = 0;
            for j in 0..rhs.digits.len() {
                let cur = res[i + j] + self.digits[i] * rhs.digits[j] + carry;
                res[i + j] = cur % BASE;
                carry = cur / BASE;
            }
            let mut pos = i + rhs.digits.len();
            while carry != 0 {
                let cur = res[pos] + carry;
                res[pos] = cur % BASE;
                carry = cur / BASE;
                pos += 1;
            }
        }
        Self::with_digits(self.sign * rhs.sign, res)
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Div for BigInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_mod(&rhs).0
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl Rem for BigInt {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.div_mod(&rhs).1
    }
}

impl RemAssign for BigInt {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone() % rhs;
    }
}
