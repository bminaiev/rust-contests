#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let a = input.usize();
    let b = input.usize();
    let s = input.string();
    let t = input.string();
    let cnt_s_0 = s.iter().filter(|&&c| c == b'0').count();
    let cnt_t_0 = t.iter().filter(|&&c| c == b'0').count();
    let prob_s = (a as f64) / ((cnt_s_0.max(a)) as f64);
    let prob_t = (b as f64) / ((cnt_t_0.max(b)) as f64);
    let mut res = 0.0;
    for i in 0..n {
        let mut cur = 0.0;
        if s[i] == b'1' || t[i] == b'1' {
            cur = 1.0;
        } else {
            cur = 1.0 - (1.0 - prob_s) * (1.0 - prob_t);
        }
        res += cur;
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
    const PROBLEM_NAME: &str = "j";
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
