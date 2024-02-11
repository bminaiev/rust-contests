//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let mut res = 0.0;
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut min_pos = 0;
    for i in 1..n {
        if a[i] < a[min_pos] {
            min_pos = i;
        }
    }
    let sum = a.iter().sum::<i64>();
    let mut cur_sum = sum;
    for start in 0..min_pos {
        let cur_res = (cur_sum as f64) / (n - start) as f64;
        if cur_res > res {
            res = cur_res;
        }
        cur_sum -= a[start];
    }
    cur_sum = sum;
    for end in (min_pos + 1..n).rev() {
        let cur_res = (cur_sum as f64) / (end + 1) as f64;
        if cur_res > res {
            res = cur_res;
        }
        cur_sum -= a[end];
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "m";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
