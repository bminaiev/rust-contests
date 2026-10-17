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
        let s = input.string();
        let mut opens = vec![];
        let mut res = 0;
        let mut closes = vec![];
        for i in 0..n {
            if s[i] == b'(' {
                opens.push(i);
                if closes.len() <= k {
                    res = res.max(opens.len() + k);
                } else {
                    let ok_pos_from = closes[closes.len() - k - 1] + 1;
                    let idx_ok =
                        binary_search_first_true(0..opens.len(), |dx| opens[dx] >= ok_pos_from);
                    let my_opens = opens.len() - idx_ok;
                    res = res.max(my_opens);
                }
            } else {
                opens.pop();
                closes.push(i);
            }
        }
        res = res.min(n / 2);
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
    const PROBLEM_NAME: &str = "e_";
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
