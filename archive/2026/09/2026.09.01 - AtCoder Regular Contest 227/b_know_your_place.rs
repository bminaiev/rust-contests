use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve_case(cnt: &[usize]) -> Option<Vec<usize>> {
    let mut cnt = cnt.to_vec();
    let n = cnt.len();
    let mut all = vec![];
    for i in 0..n {
        if cnt[i] > 0 {
            all.push(i);
        }
    }
    let mut still_alive: BTreeSet<usize> = all.iter().copied().collect();
    let mut res: Vec<usize> = vec![];
    for &cur in all.iter() {
        while res.len() < cur {
            if let Some(&last_alive_less_than_cur) = still_alive.range(..cur).next_back() {
                cnt[last_alive_less_than_cur] -= 1;
                if cnt[last_alive_less_than_cur] == 0 {
                    still_alive.remove(&last_alive_less_than_cur);
                }
                res.push(last_alive_less_than_cur);
            } else {
                return None;
            }
        }
    }
    while let Some(last) = still_alive.iter().next_back().copied() {
        cnt[last] -= 1;
        if cnt[last] == 0 {
            still_alive.remove(&last);
        }
        res.push(last);
    }
    Some(res)
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut cnt = vec![0; n];
    for i in 0..n {
        let x = input.usize();
        cnt[x] += 1;
    }
    if let Some(res) = solve_case(&mut cnt) {
        out.println("Yes");
        out.println(res);
    } else {
        out.println("No");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_know_your_place";
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
