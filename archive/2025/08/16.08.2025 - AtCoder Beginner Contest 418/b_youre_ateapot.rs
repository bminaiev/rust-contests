//{"name":"B - You're a teapot","group":"AtCoder - AtCoder Beginner Contest 418","url":"https://atcoder.jp/contests/abc418/tasks/abc418_b","interactive":false,"timeLimit":2000,"tests":[{"input":"attitude\n","output":"0.50000000000000000\n"},{"input":"ottottott\n","output":"0.66666666666666667\n"},{"input":"coffeecup\n","output":"0.00000000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BYoureATeapot"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}


//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_youre_ateapot";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
