//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn calc_one(a: &[i64], mut from_right: bool) -> i64 {
    let mut res = 0;
    let mut r = a.len();
    let mut l = 0;
    let mut order = vec![0];
    while l < r {
        if from_right {
            r -= 1;
            order.push(a[r]);
        } else {
            order.push(a[l]);
            l += 1;
        }
        from_right = !from_right;
    }
    order.push(0);
    for w in order.windows(2) {
        res += (w[1] - w[0]).max(0);
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut a = vec![];
    for _ in 0..n {
        let x = input.i64();
        let y = input.i64();
        let diff = x - y;
        a.push(diff);
    }
    a.sort();
    let res = calc_one(&a, false).max(calc_one(&a, true));
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
