//{"name":"D - cresc.","group":"AtCoder - AtCoder Regular Contest 215","url":"https://atcoder.jp/contests/arc215/tasks/arc215_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1\n","output":"5\n"},{"input":"869121 10000000\n","output":"767557322\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod7;

type Mod = Mod7;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let comb = CombinationsFact::<Mod>::new(n + m + 10);
    let mut res = Mod::ONE;
    for len in [(n + 1) / 2, (n + 2) / 2] {
        res *= comb.c(len + m, len);
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_cresc_";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
