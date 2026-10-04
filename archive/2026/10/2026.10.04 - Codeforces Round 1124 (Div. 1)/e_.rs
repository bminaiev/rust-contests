use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::math::ntt::NTT;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::misc::rand::Random;

type Mod = Mod_998_244_353;

#[target_feature(enable = "avx2")]
fn solve_case(a: &[u32]) -> Mod {
    let n = a.len();
    let combs = CombinationsFact::<Mod>::new(n * 2 + 10);
    let catalan = |n: usize| -> Mod { combs.c(2 * n, n) - combs.c(2 * n, n + 1) };

    let mut res = Mod::ZERO;
    let ways_by_len = gen_vec(n, |len| catalan(len) * catalan(n - len));
    let ntt = NTT::<Mod>::new();
    for bit in 0..20 {
        let b = gen_vec(n, |i| if ((1 << bit) & a[i]) != 0 { 1 } else { 0 });
        let mut b_pref = b.pref_sum();
        for i in 0..b_pref.len() {
            b_pref[i] &= 1;
        }
        let mut left = vec![Mod::ZERO; n + 1];
        let mut right = vec![Mod::ZERO; n + 1];
        for i in 0..=n {
            if b_pref[i] == 1 {
                left[i] = Mod::ONE;
                right[n - i] = Mod::ONE;
            } else {
                left[i] = Mod::ZERO - Mod::ONE;
                right[n - i] = Mod::ZERO - Mod::ONE;
            }
        }

        let mult = ntt.multiply(left, right);

        for len in 1..n {
            let ways = ways_by_len[len];
            let tmp_res = if b_pref[n] == 1 {
                Mod::new(n - len + 1)
            } else {
                let cnt = Mod::new(n - len + 1);
                cnt - mult[n - len]
            };
            res += ways * tmp_res * Mod::new(1 << bit);
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<u32>(n);
        let res = unsafe { solve_case(&a) };
        out.println(res);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let start = Instant::now();
        let mut rnd = Random::new(it);
        let n = 200_000;
        let a = gen_vec(n, |_| rnd.gen_range(0..1_000_000));
        let res = unsafe { solve_case(&a) };
        dbg!(res, start.elapsed());
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
