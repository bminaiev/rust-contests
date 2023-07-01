use crate::collections::last_exn::LastExn;
use crate::io::output::{Output, Writable};
use crate::misc::num_traits::{ConvSimple, HasConstants};
use std::io::Write;

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct ModRuntime {
    value: i32,
    m: i32,
}

impl ModRuntime {
    fn assert_reasonable_pair(a: &Self, b: &Self) {
        assert_ne!(a.m, 0);
        assert_ne!(a.m, 0);
        assert_eq!(a.m, b.m);
    }

    fn rev_rec(a: i32, m: i32) -> i32 {
        if a == 1 {
            return a;
        }
        ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32
    }

    #[allow(dead_code)]
    pub fn inv(self) -> Self {
        Self {
            value: Self::rev_rec(self.value, self.m),
            m: self.m,
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn new(mut x: i32, m: i32) -> Self {
        if x < 0 {
            x += m;
            if x < 0 {
                x %= m;
                x += m;
            }
        } else if x >= m {
            x -= m;
            if x >= m {
                x %= m;
            }
        }
        assert!(0 <= x && x < m);
        Self { value: x, m }
    }

    pub fn pown(self, pw: usize) -> Self {
        if pw == 0 {
            Self::new(1, self.m)
        } else if pw == 1 {
            self
        } else {
            let half = self.pown(pw / 2);
            let res = half * half;
            if pw % 2 == 0 {
                res
            } else {
                res * self
            }
        }
    }

    // TODO: `pw` should be [T], which implements "integer"
    pub fn pow_i128(self, pw: i128) -> Self {
        if pw == 0 {
            Self::new(1, self.m)
        } else if pw == 1 {
            self
        } else {
            let half = self.pow_i128(pw / 2);
            let res = half * half;
            if pw % 2 == 0 {
                res
            } else {
                res * self
            }
        }
    }

    pub fn gen_powers(base: Self, n: usize) -> Vec<Self> {
        let mut res = Vec::with_capacity(n);
        res.push(Self::ONE);
        for _ in 1..n {
            res.push(*res.last_exn() * base);
        }
        res
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl std::fmt::Display for ModRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Debug for ModRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const MAX: usize = 100;
        if self.value <= MAX as i32 {
            write!(f, "{}", self.value)
        } else if self.value >= self.m - MAX as i32 {
            write!(f, "-{}", self.m - self.value)
        } else {
            // TODO: not all ..MAX has inverse
            // for denum in 1..MAX {
            //     for num in 1..MAX {
            //         if Self::new(num as i32, self.m) / Self::new(denum as i32, self.m) == *self {
            //             return write!(f, "{}/{}", num, denum);
            //         }
            //     }
            // }
            write!(f, "(?? {} ??)", self.value)
        }
    }
}

impl std::ops::Add for ModRuntime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::assert_reasonable_pair(&self, &rhs);
        let res = self.value + rhs.value;
        if res >= self.m {
            ModRuntime::new(res - self.m, self.m)
        } else {
            ModRuntime::new(res, self.m)
        }
    }
}

impl std::ops::AddAssign for ModRuntime {
    fn add_assign(&mut self, rhs: Self) {
        Self::assert_reasonable_pair(self, &rhs);
        self.value += rhs.value;
        if self.value >= self.m {
            self.value -= self.m;
        }
    }
}

impl std::ops::Sub for ModRuntime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::assert_reasonable_pair(&self, &rhs);
        let res = self.value - rhs.value;
        if res < 0 {
            ModRuntime::new(res + self.m, self.m)
        } else {
            ModRuntime::new(res, self.m)
        }
    }
}

impl std::ops::SubAssign for ModRuntime {
    fn sub_assign(&mut self, rhs: Self) {
        Self::assert_reasonable_pair(self, &rhs);
        self.value -= rhs.value;
        if self.value < 0 {
            self.value += self.m;
        }
    }
}

impl std::ops::Mul for ModRuntime {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::assert_reasonable_pair(&self, &rhs);
        let res = (self.value as i64) * (rhs.value as i64) % (self.m as i64);
        ModRuntime::new(res as i32, self.m)
    }
}

impl std::ops::MulAssign for ModRuntime {
    fn mul_assign(&mut self, rhs: Self) {
        Self::assert_reasonable_pair(self, &rhs);
        self.value = ((self.value as i64) * (rhs.value as i64) % (self.m as i64)) as i32;
    }
}

impl std::ops::Div for ModRuntime {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::assert_reasonable_pair(&self, &rhs);
        let rhs_inv = rhs.inv();
        self * rhs_inv
    }
}

impl std::ops::DivAssign for ModRuntime {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        Self::assert_reasonable_pair(self, &rhs);
        *self *= rhs.inv();
    }
}

impl std::ops::Neg for ModRuntime {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::assert_reasonable_pair(&self, &self);
        Self::new(self.m - self.value, self.m)
    }
}

impl Writable for ModRuntime {
    fn write(&self, output: &mut Output) {
        output.write_fmt(format_args!("{}", self.value)).unwrap();
    }
}

impl HasConstants<ModRuntime> for ModRuntime {
    // This doesn't make much sense, but hope we never use
    const MAX: ModRuntime = unreachable!();
    const MIN: ModRuntime = unreachable!();
    const ZERO: ModRuntime = unreachable!();
    const ONE: ModRuntime = ModRuntime { value: 0, m: 0 };
    const TWO: ModRuntime = unreachable!();
}

impl ConvSimple<ModRuntime> for ModRuntime {
    fn from_i32(_val: i32) -> ModRuntime {
        unreachable!()
    }

    fn to_i32(self) -> i32 {
        self.value
    }

    fn to_f64(self) -> f64 {
        self.value as f64
    }
}

pub struct RuntimeModBuilder {
    modulo: i32,
}

impl RuntimeModBuilder {
    pub fn new_builder(modulo: i32) -> Self {
        Self { modulo }
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new(&self, x: i32) -> ModRuntime {
        ModRuntime {
            value: x,
            m: self.modulo,
        }
    }
}
