#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = input.vec::<i32>(n);
        a.sort();
        a.reverse();
        let mut all_ways = Mod::ONE;
        for i in 1..n {
            all_ways *= Mod::new(i);
        }
        let mut res = Mod::ZERO;
        let mut pref_sum = Mod::new(a[0]);
        for i in 1..n {
            let cur_ways = all_ways / Mod::new(i);
            res += cur_ways * (pref_sum - Mod::new(a[i]) * Mod::new(i));
            pref_sum += Mod::new(a[i]);
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
    const PROBLEM_NAME: &str = "c_";
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
