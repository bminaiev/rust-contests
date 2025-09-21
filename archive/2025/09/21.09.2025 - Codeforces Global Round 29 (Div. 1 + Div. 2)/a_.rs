//{"name":"A. Кратчайший возрастающий путь","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2\n5 6\n4 2\n1 1\n2 1\n3 3\n5 1\n5 4\n752 18572\n95152 2322\n","output":"2\n2\n3\n-1\n-1\n-1\n-1\n-1\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let x = input.i64();
        let y = input.i64();
        let res = {
            if x < y {
                2
            } else {
                if x >= y + 2 && y > 1 {
                    3
                } else {
                    -1
                }
            }
        };
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
