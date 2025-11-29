//{"name":"B. План расширения 2","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 3 3\n888\n4 5 1\n4884\n4 3 -3\n4884\n7 -7 -5\n4884884\n10 0 0\n4884884888\n1 1 1\n4\n","output":"YES\nNO\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut x = input.i64().abs();
        let mut y = input.i64().abs();
        let s = input.string();
        let mut cnt4 = 0;
        let mut cnt8 = 0;
        for x in s.iter() {
            if *x == b'4' {
                cnt4 += 1;
            } else {
                cnt8 += 1;
            }
        }
        x -= cnt8;
        y -= cnt8;
        if x < 0 {
            x = 0;
        }
        if y < 0 {
            y = 0;
        }
        let more = x + y;
        if cnt4 >= more {
            out.println("YES");
        } else {
            out.println("NO");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_2";
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
