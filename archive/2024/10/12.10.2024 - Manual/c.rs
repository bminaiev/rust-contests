//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let b = input.vec::<i64>(n);
    let c = input.vec::<i64>(n);
    let d = input.vec::<i64>(n);
    let mut cur_res = 0;
    let mut deltas = vec![];
    for i in 0..n {
        deltas.push(b[i] - a[i]);
    }
    deltas.sort();
    for i in 0..n {
        cur_res += a[i];
        cur_res += c[i];
    }
    let mut res = cur_res;
    for i in 0..n {
        cur_res += deltas[i];
        cur_res -= c[n - 1 - i];
        cur_res += d[i];
        res = res.min(cur_res);
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
