//{"name":"A - AtCoder Language","group":"AtCoder - AtCoder Beginner Contest 419","url":"https://atcoder.jp/contests/abc419/tasks/abc419_a","interactive":false,"timeLimit":2000,"tests":[{"input":"red\n","output":"SSS\n"},{"input":"atcoder\n","output":"Unknown\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAtCoderLanguage"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let s = input.string();
    if s == b"red" {
        out.println("SSS");
    } else if s == b"blue" {
        out.println("FFF");
    } else if s == b"green" {
        out.println("MMM");
    } else {
        out.println("Unknown");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_at_coder_language";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
