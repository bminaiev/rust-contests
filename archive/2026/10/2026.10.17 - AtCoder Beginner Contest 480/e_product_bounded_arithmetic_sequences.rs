#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.i128();
    let mut res = 0;
    for first in 1..=1_000_001.min(n) {
        for len in 3.. {
            let delta = binary_search_first_true(0..1_000_100_100, |delta| {
                let mut cur = 1;
                for offset in 0..len {
                    let now = first + offset * delta;
                    cur *= now;
                    if cur > n {
                        return true;
                    }
                }
                false
            });
            // dbg!(first, len, delta);
            if delta <= 1 {
                break;
            }
            res += delta - 1;
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e_product_bounded_arithmetic_sequences";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
