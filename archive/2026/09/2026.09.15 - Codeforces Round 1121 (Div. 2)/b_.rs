use std::collections::{BTreeSet, BinaryHeap};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let a = input.vec::<i64>(n);
        let mut res = i64::MIN;
        let mut smallest = BinaryHeap::new();
        let mut sum = 0;
        for i in 0..n {
            if i >= m - 1 {
                let cur = a[i] * (m as i64) - sum;
                res = res.max(cur);
            }
            smallest.push(a[i]);
            sum += a[i];
            if smallest.len() > m - 1 {
                sum -= smallest.pop().unwrap();
            }
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
