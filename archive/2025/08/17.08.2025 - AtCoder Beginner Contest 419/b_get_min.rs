//{"name":"B - Get Min","group":"AtCoder - AtCoder Beginner Contest 419","url":"https://atcoder.jp/contests/abc419/tasks/abc419_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 6\n1 7\n2\n1 1\n2\n","output":"6\n1\n"},{"input":"8\n1 5\n1 1\n1 1\n1 9\n2\n2\n1 2\n2\n","output":"1\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGetMin"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let q = input.usize();
    let mut all = vec![];
    for _ in 0..q {
        let t = input.usize();
        if t == 1 {
            let x = input.i64();
            all.push(x);
            all.sort();
        } else {
            let x = all[0];
            out.println(x);
            all.remove(0);
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_get_min";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
