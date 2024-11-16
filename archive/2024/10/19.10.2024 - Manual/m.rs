//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let R = input.i64();
        let r = input.i64();
        let mut always_ok_r = 0;
        if r * 2 > R {
            always_ok_r = R - (2 * R - 2 * r);
        } else {
            always_ok_r = R - 2 * r;
        }
        let alw_ok_r2 = always_ok_r * always_ok_r;
        let mut smallest_r2 = i64::MAX;
        let mut a = vec![];
        for _ in 0..n {
            let x = input.i64();
            let y = input.i64();
            a.push(x * x + y * y);
        }
        for i in 0..n {
            smallest_r2 = smallest_r2.min(a[i]);
        }
        let ok_r2 = smallest_r2.max(alw_ok_r2);
        let mut res = vec![];
        for i in 0..n {
            if a[i] <= ok_r2 {
                res.push(i + 1);
            }
        }
        out.println(res.len());
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
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
