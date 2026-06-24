use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n * 2);
        let mut set = BTreeSet::new();
        let mut alive = vec![true; n * 2];
        for i in 0..n * 2 {
            let pos = n * 2 - 1 - i;
            let value = a[pos];
            set.insert((value, pos));
            if i % 2 == 0 {
                let (value, pos) = set.pop_first().unwrap();
                alive[pos] = false;
            }
        }
        let mut res = vec![];
        for i in 0..2 * n {
            if alive[i] {
                res.push(i + 1);
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
    const PROBLEM_NAME: &str = "k";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
