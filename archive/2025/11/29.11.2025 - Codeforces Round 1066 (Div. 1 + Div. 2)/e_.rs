//{"name":"E. Настройка дронов","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 3\n1 1 1 1 1 1\n5 1\n1 3 2 1 4\n6 2\n1 1 1 2 3 3\n4 1\n8 8 8 8\n2 2\n1 2\n","output":"3\n4\n4\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let mut a = input.vec::<usize>(n);
        a.sort();
        let check_ok = |ops: usize| -> bool {
            let mut empty: BTreeSet<usize> = (0..3 * n + 5).collect();
            let mut all = vec![];
            for &x in a.iter().rev() {
                let first_empty_after_x = *empty.range(x..).next().unwrap();
                if first_empty_after_x - x <= ops {
                    all.push(first_empty_after_x);
                    empty.remove(&first_empty_after_x);
                } else {
                    all.push(x + ops);
                }
            }
            all.sort();
            for i in 0..(n - k) {
                if all[i] == all[i + k] {
                    return false;
                }
            }
            true
        };
        if check_ok(0) {
            out.println(0);
        } else {
            let x = binary_search_first_true(0..2 * n + 5, check_ok);
            out.println(x);
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
    const PROBLEM_NAME: &str = "e_";
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
