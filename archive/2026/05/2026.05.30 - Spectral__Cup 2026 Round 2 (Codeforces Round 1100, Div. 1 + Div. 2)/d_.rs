#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_last_true;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);
        let b = input.vec::<usize>(n);
        let res = binary_search_last_true(0..2 * n + 10, |check| {
            let mut cnt = [0; 3];
            let mut last_was_zero = false;
            for i in 0..n {
                let cur = [
                    if a[i] >= check { 1 } else { 0 },
                    if b[i] >= check { 1 } else { 0 },
                ];
                let cur: usize = cur.iter().sum();
                if cur == 1 {
                    continue;
                }
                if cur == 0 && last_was_zero {
                    continue;
                }
                cnt[cur] += 1;
                last_was_zero = cur == 0;
            }
            cnt[2] > cnt[0]
        })
        .unwrap();
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
