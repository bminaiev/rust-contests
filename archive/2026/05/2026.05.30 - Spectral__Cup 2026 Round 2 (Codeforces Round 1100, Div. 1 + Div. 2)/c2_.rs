#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn best_pos(a: &[i64]) -> Option<usize> {
    let sum: i64 = a.iter().sum();
    let mut best = (None, sum);
    let mut pref_sum = 0;
    let mut pref_pos_sum = 0;
    for i in 0..a.len() {
        if a[i] > 0 {
            let candidate = sum - pref_sum - 2 * a[i] + pref_pos_sum;
            if candidate > best.1 {
                best = (Some(i), candidate);
            }
        }
        pref_sum += a[i];
        pref_pos_sum += a[i].abs();
    }
    best.0
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n);
        let mut res = vec![];
        if let Some(mid) = best_pos(&a) {
            let mut swapped = false;
            for i in (0..mid).rev() {
                let positive = (a[i] > 0) ^ swapped;
                if positive {
                    res.push(i + 1);
                    swapped = !swapped;
                }
            }
            res.push(mid + 1);
        }
        out.println(res.len());
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
    const PROBLEM_NAME: &str = "c2_";
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
