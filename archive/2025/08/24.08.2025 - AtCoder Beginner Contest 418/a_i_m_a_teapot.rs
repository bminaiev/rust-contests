//{"name":"A - I'm a teapot","group":"AtCoder - AtCoder Beginner Contest 418","url":"https://atcoder.jp/contests/abc418/tasks/abc418_a","interactive":false,"timeLimit":2000,"tests":[{"input":"8\ngreentea\n","output":"Yes\n"},{"input":"6\ncoffee\n","output":"No\n"},{"input":"3\ntea\n","output":"Yes\n"},{"input":"1\nt\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let _n = input.usize();
    let s = input.string();
    if s.ends_with(b"tea") {
        out.println("Yes");
    } else {
        out.println("No");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_i_m_a_teapot";
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
