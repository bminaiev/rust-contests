//{"name":"C - King's Summit","group":"AtCoder - AtCoder Beginner Contest 419","url":"https://atcoder.jp/contests/abc419/tasks/abc419_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3\n5 1\n8 1\n","output":"3\n"},{"input":"5\n6 7\n6 7\n6 7\n6 7\n6 7\n","output":"0\n"},{"input":"6\n91 999999986\n53 999999997\n32 999999932\n14 999999909\n49 999999985\n28 999999926\n","output":"44\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKingsSummit"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut all_x = vec![];
    let mut all_y = vec![];
    for _ in 0..n {
        let x = input.i64();
        let y = input.i64();
        all_x.push(x);
        all_y.push(y);
    }
    all_x.sort();
    all_y.sort();
    let max_d = (all_x[n - 1] - all_x[0]).max(all_y[n - 1] - all_y[0]);
    out.println((max_d + 1) / 2);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c_kings_summit";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
