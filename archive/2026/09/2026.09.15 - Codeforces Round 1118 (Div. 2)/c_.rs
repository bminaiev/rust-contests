#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn ask_at_least(input: &mut Input, out: &mut Output, a: usize, b: usize, d: usize) -> bool {
    out.println(format!("? {} {} {}", a + 1, b + 1, d));
    out.flush();
    input.i64() == 1
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut cur_dist = 0;
        let mut first = 0;
        for v in 1..n {
            while ask_at_least(input, out, 0, v, cur_dist + 1) {
                cur_dist += 1;
                first = v;
            }
        }
        let mut second = 0;
        for u in 0..n {
            if u == first {
                continue;
            }
            while ask_at_least(input, out, u, first, cur_dist + 1) {
                cur_dist += 1;
                second = u;
            }
        }
        out.println(format!("! {} {} {}", first + 1, second + 1, cur_dist));
        out.flush();
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

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
