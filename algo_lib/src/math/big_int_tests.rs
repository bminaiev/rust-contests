#[cfg(test)]
pub mod tests {
    use crate::math::big_int::BigInt;

    #[test]
    fn constructors_and_formatting() {
        assert_eq!(BigInt::new(0_u8).to_string(), "0");
        assert_eq!(BigInt::new(-17_i8).to_string(), "-17");
        assert_eq!(BigInt::new(17_usize).to_string(), "17");
        assert_eq!(BigInt::new(i128::MIN).to_string(), i128::MIN.to_string());
        assert_eq!(BigInt::new(u128::MAX).to_string(), u128::MAX.to_string());
    }

    #[test]
    fn add_and_sub() {
        assert_eq!(
            (BigInt::new(1_000_000_000_i64) - BigInt::new(1_i64)).to_string(),
            "999999999"
        );
        assert_eq!((BigInt::new(-7_i64) + BigInt::new(3_i64)).to_string(), "-4");
        assert_eq!((BigInt::new(3_i64) - BigInt::new(7_i64)).to_string(), "-4");
        assert_eq!(
            (BigInt::new(-7_i64) - BigInt::new(3_i64)).to_string(),
            "-10"
        );
    }

    #[test]
    fn mul() {
        let a = 123_456_789_012_345_678_i128;
        let b = 987_654_321_i128;
        assert_eq!(
            (BigInt::new(a) * BigInt::new(b)).to_string(),
            (a * b).to_string()
        );
        assert_eq!(
            (BigInt::new(-123_456_789_i64) * BigInt::new(0_i64)).to_string(),
            "0"
        );
    }

    #[test]
    fn div_and_rem() {
        let a = 1_234_567_890_123_456_789_i128;
        let b = 987_654_321_i128;
        for lhs in [a, -a] {
            for rhs in [b, -b] {
                let big_lhs = BigInt::new(lhs);
                let big_rhs = BigInt::new(rhs);
                let (q, r) = big_lhs.div_mod(&big_rhs);
                assert_eq!(q.to_string(), (lhs / rhs).to_string());
                assert_eq!(r.to_string(), (lhs % rhs).to_string());
                assert_eq!(
                    (BigInt::new(lhs) / BigInt::new(rhs)).to_string(),
                    (lhs / rhs).to_string()
                );
                assert_eq!(
                    (BigInt::new(lhs) % BigInt::new(rhs)).to_string(),
                    (lhs % rhs).to_string()
                );
            }
        }
    }

    #[test]
    fn small_exhaustive() {
        for lhs in -100_i128..=100 {
            for rhs in -100_i128..=100 {
                assert_eq!(
                    (BigInt::new(lhs) + BigInt::new(rhs)).to_string(),
                    (lhs + rhs).to_string()
                );
                assert_eq!(
                    (BigInt::new(lhs) - BigInt::new(rhs)).to_string(),
                    (lhs - rhs).to_string()
                );
                assert_eq!(
                    (BigInt::new(lhs) * BigInt::new(rhs)).to_string(),
                    (lhs * rhs).to_string()
                );
                if rhs != 0 {
                    assert_eq!(
                        (BigInt::new(lhs) / BigInt::new(rhs)).to_string(),
                        (lhs / rhs).to_string()
                    );
                    assert_eq!(
                        (BigInt::new(lhs) % BigInt::new(rhs)).to_string(),
                        (lhs % rhs).to_string()
                    );
                }
            }
        }
    }

    #[test]
    fn div_large_unsigned() {
        let a = u128::MAX;
        let b = 1_000_000_000_u128;
        let (q, r) = BigInt::new(a).div_mod(&BigInt::new(b));
        assert_eq!(q.to_string(), (a / b).to_string());
        assert_eq!(r.to_string(), (a % b).to_string());
    }
}
