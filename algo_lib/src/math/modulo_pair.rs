use crate::collections::last_exn::LastExn;
use crate::io::input::{Input, Readable};
use crate::math::modulo::{ModWithValue, Value, Value7, Value_998_244_353};
use crate::misc::num_traits::{ConvSimple, HasConstants, Number};

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct ModPair<M1, M2>(ModWithValue<M1>, ModWithValue<M2>)
where
    M1: Value,
    M2: Value;

impl<M1, M2> ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    #[allow(unused)]
    pub const ZERO: Self = Self(ModWithValue::ZERO, ModWithValue::ZERO);

    #[allow(unused)]
    pub const ONE: Self = Self(ModWithValue::ONE, ModWithValue::ONE);

    #[allow(unused)]
    pub const TWO: Self = Self(ModWithValue::TWO, ModWithValue::TWO);

    pub fn value(&self) -> (i32, i32) {
        (self.0.value(), self.1.value())
    }

    #[allow(dead_code)]
    pub fn new<T: Number>(x: T) -> Self {
        Self(ModWithValue::<M1>::new(x), ModWithValue::<M2>::new(x))
    }

    pub fn pown(self, pw: usize) -> Self {
        Self(self.0.pown(pw), self.1.pown(pw))
    }

    pub fn gen_powers(base: Self, n: usize) -> Vec<Self> {
        let mut res = Vec::with_capacity(n);
        res.push(Self::ONE);
        for _ in 1..n {
            res.push(*res.last_exn() * base);
        }
        res
    }
}

// impl<M1, M2> std::fmt::Display for ModPair<M1, M2>
// where
//     M1: Value,
//     M2: Value,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl<M1, M2> std::fmt::Debug for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.0, self.1)
    }
}

impl<M1, M2> std::ops::Add for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<M1, M2> std::ops::AddAssign for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<M1, M2> std::ops::Sub for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<M1, M2> std::ops::SubAssign for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<M1, M2> std::ops::Mul for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<M1, M2> std::ops::MulAssign for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

impl<M1, M2> std::ops::Div for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl<M1, M2> std::ops::DivAssign for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}

// impl<M1, M2> Writable for ModPair<M1, M2>
// where
//     M1: Value,
//     M2: Value,
// {
//     fn write(&self, output: &mut Output) {
//         output.write_fmt(format_args!("{}", self.0)).unwrap();
//     }
// }

impl<M1, M2> Readable for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    fn read(input: &mut Input) -> Self {
        let i32 = input.i32();
        Self::new(i32)
    }
}

impl<M1, M2> HasConstants<ModPair<M1, M2>> for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    // This doesn't make much sense, but hope we never use
    const MAX: ModPair<M1, M2> = ModPair::ZERO;
    const MIN: ModPair<M1, M2> = ModPair::ZERO;
    const ZERO: ModPair<M1, M2> = ModPair::ZERO;
    const ONE: ModPair<M1, M2> = ModPair::ONE;
    const TWO: ModPair<M1, M2> = ModPair::TWO;
}

impl<M1, M2> ConvSimple<ModPair<M1, M2>> for ModPair<M1, M2>
where
    M1: Value,
    M2: Value,
{
    fn from_i32(val: i32) -> ModPair<M1, M2> {
        ModPair::new(val)
    }

    fn to_i32(self) -> i32 {
        self.0.to_i32()
    }

    fn to_f64(self) -> f64 {
        self.0.to_f64()
    }
}

pub type ModPair998_007 = ModPair<Value_998_244_353, Value7>;
