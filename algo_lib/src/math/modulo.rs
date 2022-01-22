use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::num_traits::{ConvI32, HasConstants, Number};
use std::io::Write;
use std::marker::PhantomData;

pub trait Value: Clone + Copy + Eq + Default + Ord {
    fn val() -> i32;
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct ModWithValue<M>(i32, PhantomData<M>)
where
    M: Value;

impl<M> ModWithValue<M>
where
    M: Value,
{
    #[allow(unused)]
    pub const ZERO: Self = Self(0, PhantomData);

    #[allow(unused)]
    pub const ONE: Self = Self(1, PhantomData);

    #[allow(unused)]
    pub const TWO: Self = Self(2, PhantomData);

    fn rev_rec(a: i32, m: i32) -> i32 {
        if a == 1 {
            return a;
        }
        return ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32;
    }

    #[allow(dead_code)]
    fn inv(self) -> Self {
        ModWithValue(Self::rev_rec(self.0, M::val()), PhantomData)
    }

    #[allow(dead_code)]
    pub fn new(mut x: i32) -> Self {
        if x < 0 {
            x += M::val();
            if x < 0 {
                x %= M::val();
                x += M::val();
            }
        } else if x >= M::val() {
            x -= M::val();
            if x >= M::val() {
                x %= M::val();
            }
        }
        assert!(0 <= x && x < M::val());
        Self(x, PhantomData)
    }

    pub fn pown(self, pw: usize) -> Self {
        if pw == 0 {
            Self::ONE
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
}

impl<M> std::fmt::Display for ModWithValue<M>
where
    M: Value,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M> std::fmt::Debug for ModWithValue<M>
where
    M: Value + Copy + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const MAX: usize = 100;
        if self.0 <= MAX as i32 {
            write!(f, "{}", self.0)
        } else if self.0 >= M::val() - MAX as i32 {
            write!(f, "-{}", M::val() - self.0)
        } else {
            for denum in 1..MAX {
                for num in 1..MAX {
                    if Self(num as i32, PhantomData) / Self(denum as i32, PhantomData) == *self {
                        return write!(f, "{}/{}", num, denum);
                    }
                }
            }
            write!(f, "(?? {} ??)", self.0)
        }
    }
}

impl<M> std::ops::Add for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let res = self.0 + rhs.0;
        if res >= M::val() {
            ModWithValue(res - M::val(), PhantomData)
        } else {
            ModWithValue(res, PhantomData)
        }
    }
}

impl<M> std::ops::AddAssign for ModWithValue<M>
where
    M: Value,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        if self.0 >= M::val() {
            self.0 -= M::val();
        }
    }
}

impl<M> std::ops::Sub for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let res = self.0 - rhs.0;
        if res < 0 {
            ModWithValue(res + M::val(), PhantomData)
        } else {
            ModWithValue(res, PhantomData)
        }
    }
}

impl<M> std::ops::SubAssign for ModWithValue<M>
where
    M: Value,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        if self.0 < 0 {
            self.0 += M::val();
        }
    }
}

impl<M> std::ops::Mul for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = (self.0 as i64) * (rhs.0 as i64) % (M::val() as i64);
        ModWithValue(res as i32, PhantomData)
    }
}

impl<M> std::ops::MulAssign for ModWithValue<M>
where
    M: Value,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = ((self.0 as i64) * (rhs.0 as i64) % (M::val() as i64)) as i32;
    }
}

impl<M> std::ops::Div for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inv = rhs.inv();
        self * rhs_inv
    }
}

impl<M> std::ops::DivAssign for ModWithValue<M>
where
    M: Value,
{
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl<M> Writable for ModWithValue<M>
where
    M: Value,
{
    fn write(&self, output: &mut Output) {
        output.write_fmt(format_args!("{}", self.0)).unwrap();
    }
}

impl<M> Readable for ModWithValue<M>
where
    M: Value,
{
    fn read(input: &mut Input) -> Self {
        let i32 = input.i32();
        Self::new(i32)
    }
}

impl<M> HasConstants<ModWithValue<M>> for ModWithValue<M>
where
    M: Value,
{
    // This doesn't make much sense, but hope we never use
    const MAX: ModWithValue<M> = ModWithValue::ZERO;
    const MIN: ModWithValue<M> = ModWithValue::ZERO;
    const ZERO: ModWithValue<M> = ModWithValue::ZERO;
    const ONE: ModWithValue<M> = ModWithValue::ONE;
    const TWO: ModWithValue<M> = ModWithValue::TWO;
}

impl<M> ConvI32<ModWithValue<M>> for ModWithValue<M>
where
    M: Value,
{
    fn from_i32(val: i32) -> ModWithValue<M> {
        ModWithValue::new(val)
    }

    fn to_i32(self) -> i32 {
        self.0
    }
}

pub trait ConstValue: Value + Copy {
    const VAL: i32;
}

impl<V: ConstValue> Value for V {
    fn val() -> i32 {
        Self::VAL
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Value7();
impl ConstValue for Value7 {
    const VAL: i32 = 1_000_000_007;
}
pub type Mod7 = ModWithValue<Value7>;

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Value9();
impl ConstValue for Value9 {
    const VAL: i32 = 1_000_000_009;
}
pub type Mod9 = ModWithValue<Value9>;

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct Value_998_244_353();
impl ConstValue for Value_998_244_353 {
    const VAL: i32 = 998_244_353;
}
#[allow(non_camel_case_types)]
pub type Mod_998_244_353 = ModWithValue<Value_998_244_353>;

pub trait ModuloTrait: Number {
    fn mod_value() -> i32;
}

impl<V: Value> ModuloTrait for ModWithValue<V> {
    fn mod_value() -> i32 {
        V::val()
    }
}
