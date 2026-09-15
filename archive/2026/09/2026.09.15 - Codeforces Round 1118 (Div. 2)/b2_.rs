use std::cmp::min;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let mut cnt = vec![0; m + 1];
        for _ in 0..n {
            cnt[input.usize()] += 1;
        }
        for i in 1..=m {
            cnt[i] += cnt[i - 1];
        }
        const MX: usize = 20;
        let mut res = vec![0; MX];
        for x in 1..=m {
            let mut tmp_res = [0; MX];
            for mult in 1..=m / x {
                let from = x * mult;
                let to = (x * (mult + 1)).min(m + 1);
                let cnt = cnt[to - 1] - cnt[from - 1];
                for lvl in 0..MX {
                    let max_splits = 1 << (lvl + 1);
                    let cur_res = min(mult, max_splits - 1);
                    tmp_res[lvl] += cnt * cur_res;
                }
            }
            for lvl in 0..MX {
                let max_splits = 1 << (lvl + 1);
                let need = x * max_splits;
                if need <= m {
                    tmp_res[lvl] += cnt[need] - cnt[need - 1];
                }
            }
            for i in 0..MX {
                res[i] = res[i].max(tmp_res[i]);
            }
        }
        res.truncate(m);
        let last = res[res.len() - 1];
        while res.len() < m {
            res.push(last);
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
    const PROBLEM_NAME: &str = "b2_";
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
