//{"name":"B - Puzzle of Lamps","group":"AtCoder - AtCoder Regular Contest 177","url":"https://atcoder.jp/contests/arc177/tasks/arc177_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n01100\n","output":"4\nAAAB\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPuzzleOfLamps"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut res = vec![];
    for i in (0..n).rev() {
        if s[i] == b'1' {
            res.extend(vec![b'A'; i + 1]);
            res.extend(vec![b'B'; i]);
        }
    }
    let s = String::from_utf8(res).unwrap();
    out.println(s.len());
    out.println(s);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_puzzle_of_lamps";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
