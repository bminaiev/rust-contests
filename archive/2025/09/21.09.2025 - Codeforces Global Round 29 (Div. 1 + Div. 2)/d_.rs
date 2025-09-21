//{"name":"D. Игра на массиве","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n2 1 1\n5\n3 3 3 5 5\n4\n9 9 9 9\n","output":"3 1\n10 9\n20 16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::BTreeMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n);
        let mut cnts = BTreeMap::new();
        let mut sum = 0;
        for &x in a.iter() {
            if x % 2 == 0 {
                sum += x;
            } else {
                *cnts.entry(x).or_insert(0) += 1i64;
                sum += x - 1;
            }
        }
        let mut extra = vec![];
        for (&x, &cnt) in cnts.iter() {
            extra.push(cnt);
        }
        extra.sort();
        extra.reverse();
        assert!(sum % 2 == 0);
        let mut res = vec![sum / 2; 2];
        for i in 0..extra.len() {
            res[i % 2] += extra[i];
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
    const PROBLEM_NAME: &str = "d_";
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
