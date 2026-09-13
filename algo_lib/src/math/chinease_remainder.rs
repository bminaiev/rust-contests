use std::ops::Rem;

use crate::{math::gcd::gcd, misc::num_traits::Number};

fn add_mod<N: Number + Rem<Output = N>>(a: N, b: N, modulus: N) -> N {
    if a >= modulus - b {
        a - (modulus - b)
    } else {
        a + b
    }
}

fn mul_mod<N: Number + Rem<Output = N>>(mut a: N, mut b: N, modulus: N) -> N {
    let mut result = N::ZERO;
    a = a % modulus;
    while b > N::ZERO {
        if b % N::TWO != N::ZERO {
            result = add_mod(result, a, modulus);
        }
        b /= N::TWO;
        if b > N::ZERO {
            a = add_mod(a, a, modulus);
        }
    }
    result
}

fn mod_inverse<N: Number + Rem<Output = N>>(value: N, modulus: N) -> Option<N> {
    if modulus == N::ONE {
        return Some(N::ZERO);
    }
    let (mut r0, mut r1) = (modulus, value % modulus);
    let (mut x0, mut x1) = (N::ZERO, N::ONE);
    while r1 != N::ZERO {
        let quotient = r0 / r1;
        (r0, r1) = (r1, r0 % r1);
        let sub = mul_mod(quotient % modulus, x1, modulus);
        let x2 = if x0 >= sub {
            x0 - sub
        } else {
            modulus - (sub - x0)
        };
        (x0, x1) = (x1, x2);
    }
    (r0 == N::ONE).then_some(x0)
}

// finds x such that x = a[i] (mod m[i]) for all i
pub fn chinease_remainder<N: Number + Rem<Output = N>>(a: &[N], m: &[N]) -> Option<N> {
    (a.len() == m.len()).then_some(())?;
    let (mut result, mut step) = (N::ZERO, N::ONE);
    for (&value, &modulus) in a.iter().zip(m) {
        (modulus > N::ZERO).then_some(())?;
        let value = (value % modulus + modulus) % modulus;
        let g = gcd(step, modulus);
        let result_mod = result % modulus;
        (value % g == result_mod % g).then_some(())?;

        let reduced_modulus = modulus / g;
        let difference = if value >= result_mod {
            (value - result_mod) / g
        } else {
            reduced_modulus - (result_mod - value) / g
        };
        let inverse = mod_inverse((step / g) % reduced_modulus, reduced_modulus)?;
        let multiplier = mul_mod(difference, inverse, reduced_modulus);
        result += step * multiplier;
        step *= reduced_modulus;
    }
    Some(result)
}
