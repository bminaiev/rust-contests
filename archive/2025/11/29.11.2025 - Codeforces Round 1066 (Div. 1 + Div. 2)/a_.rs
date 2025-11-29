//{"name":"A. Эквилибриум подземелья","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 2\n5\n1 1 2 2 3\n10\n1 2 3 2 4 4 4 4 5 2\n1\n0\n","output":"0\n2\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut cnt = HashMap::new();
        for _ in 0..n {
            let x = input.usize();
            *cnt.entry(x).or_insert(0) += 1;
        }
        let mut res = 0;
        for (k, v) in cnt {
            if v >= k {
                res += v - k;
            } else {
                res += v;
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_";
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
