#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::frac::FracT;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::binary_search::binary_search_first_true;

type Frac = FracT<i64>;
type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n);
        let b = input.vec::<i64>(n);
        let mut all = vec![];
        for i in 0..n {
            for j in i + 1..n {
                all.push(Frac::new(a[i], a[j]));
            }
        }
        all.sort();
        let mut res = Mod::ZERO;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let need = Frac::new(b[i], b[j]);
                let from = binary_search_first_true(0..all.len(), |i| all[i] > need);
                res += Mod::new(all.len() - from);
            }
        }
        if n != 1 {
            res /= Mod::new(n as i64) * Mod::new((n - 1) as i64);
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
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
