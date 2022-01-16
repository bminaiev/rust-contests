use crate::misc::num_traits::Number;

#[allow(dead_code)]
fn extended_gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if a == 0 {
        *x = 0;
        *y = 1;
        return b;
    }
    let mut x1 = 0;
    let mut y1 = 0;
    let d = extended_gcd(b % a, a, &mut x1, &mut y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    d
}

///
///
/// Find any solution to equation A*x + B*y = C
///
/// Returns [false] if [C] is not divisible by gcd(A, B)
///
#[allow(dead_code)]
pub fn diophantine(a: i64, b: i64, c: i64, x0: &mut i64, y0: &mut i64, g: &mut i64) -> bool {
    *g = extended_gcd(a.abs(), b.abs(), x0, y0);
    if c % *g != 0 {
        return false;
    }
    *x0 *= c / *g;
    *y0 *= c / *g;
    if a < 0 {
        *x0 *= -1;
    }
    if b < 0 {
        *y0 *= -1;
    }
    true
}

#[allow(dead_code)]
pub fn gcd<T>(x: T, y: T) -> T
where
    T: Number + std::ops::Rem<Output = T>,
{
    if x == T::ZERO {
        y
    } else {
        gcd(y % x, x)
    }
}

pub fn lcm<T>(x: T, y: T) -> T
where
    T: Number + std::ops::Rem<Output = T>,
{
    x / gcd(x, y) * y
}
