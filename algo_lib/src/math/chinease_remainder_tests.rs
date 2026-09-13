#[cfg(test)]
pub mod tests {
    use crate::math::chinease_remainder::chinease_remainder;

    #[test]
    fn coprime_moduli() {
        assert_eq!(chinease_remainder(&[2_i64, 3, 2], &[3, 5, 7]), Some(23));
    }

    #[test]
    fn non_coprime_moduli() {
        assert_eq!(chinease_remainder(&[2_i64, 6], &[4, 8]), Some(6));
        assert_eq!(chinease_remainder(&[1_i64, 2], &[2, 4]), None);
    }

    #[test]
    fn normalizes_residues() {
        assert_eq!(chinease_remainder(&[-1_i64, 1], &[5, 2]), Some(9));
        assert_eq!(chinease_remainder(&[17_u64, 3], &[5, 7]), Some(17));
    }

    #[test]
    fn large_moduli() {
        assert_eq!(
            chinease_remainder(&[28_i128, 223_092_869], &[29, 223_092_870]),
            Some(6_469_693_229)
        );
    }

    #[test]
    fn validates_input() {
        assert_eq!(chinease_remainder::<i64>(&[], &[]), Some(0));
        assert_eq!(chinease_remainder(&[1_i64], &[]), None);
        assert_eq!(chinease_remainder(&[1_i64], &[0]), None);
        assert_eq!(chinease_remainder(&[1_i64], &[-3]), None);
    }
}
