mod tests {
    extern crate test;
    use self::test::Bencher;
    use crate::math::modulo::{Mod7, Mod9};
    use crate::math::modulo_runtime::ModRuntime;

    // around 57 ns
    #[bench]
    fn test_sum(b: &mut Bencher) {
        type Mod = Mod9;
        let x = Mod::new(test::black_box(23));
        let y = Mod::new(test::black_box(600_000_000));
        b.iter(|| (0..100).fold(x, |init, _iter| init + y));
    }

    // around 100 ns
    #[bench]
    fn test_sum_runtime(b: &mut Bencher) {
        type Mod = ModRuntime;
        let m = test::black_box(1_000_000_007);
        let x = Mod::new(test::black_box(23), m);
        let y = Mod::new(test::black_box(600_000_000), m);
        b.iter(|| (0..100).fold(x, |init, _iter| init + y));
    }

    // around 340 ns
    #[bench]
    fn test_mult(b: &mut Bencher) {
        type Mod = Mod9;
        let x = Mod::new(test::black_box(23));
        let y = Mod::new(test::black_box(600_000_000));
        b.iter(|| (0..100).fold(x, |init, _iter| init * y));
    }

    // around 491
    #[bench]
    fn test_mult_runtime(b: &mut Bencher) {
        type Mod = ModRuntime;
        let m = test::black_box(1_000_000_007);
        let x = Mod::new(test::black_box(23), m);
        let y = Mod::new(test::black_box(600_000_000), m);
        b.iter(|| (0..100).fold(x, |init, _iter| init * y));
    }
}
