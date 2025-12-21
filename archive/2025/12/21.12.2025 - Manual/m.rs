//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::CombinationsFact;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut xx = vec![];
    let mut yy = vec![];
    const MD: i64 = 998_244_353;
    for _ in 0..n {
        let x = (input.i64() + MD) % MD;
        let y = (input.i64() + MD) % MD;
        xx.push(Mod::new(x));
        yy.push(Mod::new(y));
    }
    let cnk = CombinationsFact::<Mod>::new(n + 1);
    let mult = vec![Mod::ZERO; n + 1];
    for d in 1..n {
        // let left =
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "m";
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
