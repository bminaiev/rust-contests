#[cfg(test)]
mod tests {
    use crate::misc::rand::Random;
    use crate::strings::suffix_array::SuffixArray;

    #[test]
    fn simple() {
        const MAX: usize = 30;
        for test in 0..100 {
            let mut rnd = Random::new(787788 + test);
            let n = rnd.gen_range(1..MAX);
            let alph_size = rnd.gen_range(1..10);
            let str = rnd.gen_vec(n, b'a'..b'a' + alph_size);
            let _suf_array = SuffixArray::new(str);
        }
    }

    // Small alphabet + larger n exercises deep SA-IS recursion; correctness is
    // checked by the debug assertions inside `SuffixArray::new`.
    #[test]
    fn stress() {
        const MAX: usize = 2000;
        for test in 0..20 {
            let mut rnd = Random::new(123 + test);
            let n = rnd.gen_range(1..MAX);
            let alph_size = rnd.gen_range(1..4);
            let str = rnd.gen_vec(n, b'a'..b'a' + alph_size);
            let _suf_array = SuffixArray::new(str);
        }
    }
}
