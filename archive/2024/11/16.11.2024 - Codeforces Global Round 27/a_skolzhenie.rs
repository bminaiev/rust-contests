//{"name":"A. Скольжение","group":"Codeforces - Codeforces Global Round 27","url":"https://codeforces.com/contest/2035/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 3 1 2\n2 2 2 1\n1 1 1 1\n1000000 1000000 1 1\n","output":"6\n1\n0\n1999998000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASkolzhenie"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.i64();
    let m = input.i64();
    let r = input.i64() - 1;
    let c = input.i64() - 1;
    let before = r * m + c;
    let after = n * m - before - 1;
    let mut res = after;
    let next_rows = n - r - 1;
    res += next_rows * (m - 1);
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
    const PROBLEM_NAME: &str = "a_skolzhenie";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
