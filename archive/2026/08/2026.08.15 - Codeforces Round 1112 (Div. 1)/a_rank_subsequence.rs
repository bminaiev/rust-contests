#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut left = vec![];
        let mut right = vec![];
        let mut left2 = vec![];
        let mut right2 = vec![];
        for _ in 0..n {
            left.push(input.i32());
            right.push(input.i32());
            left2.push(input.i32());
            right2.push(input.i32());
        }
        let mut ans = 0;
        for total in 1..=n {
            let mut cur_cnt = 0;
            for i in 0..n {
                if left[i] <= cur_cnt as i32 + 1 && cur_cnt as i32 + 1 <= right[i] {
                    continue;
                }
                let rev_i = (total - cur_cnt) as i32;
                if left2[i] <= rev_i && rev_i <= right2[i] {
                    continue;
                }
                cur_cnt += 1;
                if cur_cnt >= total {
                    break;
                }
            }
            if cur_cnt >= total {
                ans = cur_cnt;
            }
        }
        out.println(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_rank_subsequence";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
