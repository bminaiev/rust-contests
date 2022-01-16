#[cfg(test)]
mod tests {
    use crate::misc::rand::Random;
    use crate::strings::suffix_array::SuffixArray;

    #[test]
    fn simple() {
        const MAX: usize = 100;
        for test in 0..1000 {
            let mut rnd = Random::new(787788 + test);
            let n = rnd.gen_range(1..MAX);
            let alph_size = rnd.gen_range(1..10);
            let str = rnd.gen_vec(n, b'a'..b'a' + alph_size);
            let _suf_array = SuffixArray::new(str);
        }
    }
}
