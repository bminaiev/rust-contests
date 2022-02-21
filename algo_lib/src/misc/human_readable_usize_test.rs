#[cfg(test)]
pub mod tests {
    use crate::misc::human_readable_usize::HumanReadableUsize;
    use expect_test::expect;

    #[test]
    pub fn simple() {
        let mut number = 1234567890usize;
        let mut res = String::new();
        while number > 0 {
            res += &format!("{:?}, ", HumanReadableUsize(number));
            number /= 10;
        }
        let expected = expect![[r#"1234M, 123M, 12M, 1.2M, 123K, 12K, 1.2K, 123, 12, 1, "#]];
        expected.assert_eq(&res);
    }
}
