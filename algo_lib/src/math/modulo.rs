use std::marker::PhantomData;

pub trait Value {
    fn val() -> i32;
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
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
        } else if x >= M::val() {
            x -= M::val();
        }
        assert!(0 <= x && x < M::val());
        Self(x, PhantomData)
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

pub trait ConstValue: Value {
    const VAL: i32;
}

impl<V: ConstValue> Value for V {
    fn val() -> i32 {
        Self::VAL
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Value7();
impl ConstValue for Value7 {
    const VAL: i32 = 1_000_000_007;
}
pub type Mod7 = ModWithValue<Value7>;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Value9();
impl ConstValue for Value9 {
    const VAL: i32 = 1_000_000_009;
}
pub type Mod9 = ModWithValue<Value9>;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct Value_998_244_353();
impl ConstValue for Value_998_244_353 {
    const VAL: i32 = 998_244_353;
}
#[allow(non_camel_case_types)]
pub type Mod_998_244_353 = ModWithValue<Value_998_244_353>;

#[cfg(test)]
mod tests {
    use crate::math::modulo::*;

    type Mod = Mod7;

    #[test]
    fn add() {
        let x = Mod::new(1);
        let y = Mod::new(2);
        assert_eq!(format!("{}", x + y), "3");
    }

    #[test]
    fn sub() {
        let x = Mod::new(1);
        let y = Mod::new(2);
        assert_eq!(format!("{}", x - y), "1000000006");
        assert_eq!(format!("{:?}", x - y), "-1");
    }

    #[test]
    fn mul() {
        let x = Mod::new(3);
        let y = Mod::new(5);
        assert_eq!(format!("{}", x * y), "15");
    }

    #[test]
    fn div() {
        let x = Mod::new(3);
        let y = Mod::new(5);
        assert_eq!(format!("{}", x / y), "200000002");
        assert_eq!(format!("{:?}", x / y), "3/5");
    }

    #[test]
    fn div_assign() {
        let mut x = Mod::new(3);
        let y = Mod::new(5);
        x /= y;
        assert_eq!(format!("{}", x), "200000002");
        assert_eq!(format!("{:?}", x), "3/5");
    }

    #[test]
    fn dbg_format() {
        let x = Mod::new(1) / Mod::new(2);
        let y = Mod::new(1) / Mod::new(3);
        assert_eq!(format!("{}", x + y), "833333340");
        assert_eq!(format!("{:?}", x + y), "5/6");
    }

    #[test]
    fn dbg_format_big() {
        let x = Mod::new(123) / Mod::new(457);
        assert_eq!(format!("{:?}", x), "(?? 262582059 ??)");
    }

    #[test]
    fn dbg_format_more() {
        assert_eq!(format!("{:?}", Mod::new(1)), "1");
        assert_eq!(format!("{:?}", Mod::new(3)), "3");
        assert_eq!(format!("{:?}", Mod::new(-5)), "-5");
    }

    #[test]
    fn consts() {
        let one = Mod::ONE - Mod::ZERO;
        assert_eq!(format!("{:?}", one), "1");
    }
}
