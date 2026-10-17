#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let mut s = vec![];
    for _ in 0..n {
        s.push(input.string());
    }
    let mut scores = vec![0; n];
    for _ in 0..m {
        let vote = input.string();
        let mut f = false;
        for i in 0..n {
            if vote == s[i] {
                scores[i] += 1;
                f = true;
            }
        }
        if !f {
            let mut ids = vec![];
            for i in 0..n {
                if s[i].len() == vote.len() {
                    let mut cnt_diff = 0;
                    for j in 0..vote.len() {
                        if s[i][j] != vote[j] {
                            cnt_diff += 1;
                        }
                    }
                    if cnt_diff == 1 {
                        ids.push(i);
                    }
                }
            }
            if ids.len() == 1 {
                scores[ids[0]] += 1;
            }
        }
    }
    out.println(scores);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_invalid_vote";
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
