#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let per_rock = input.usize();
        let per_round = input.usize();
        const MAX_TIME: usize = 50_000;
        const MAX_ROCKS: usize = 5000;
        // have time -> max options to distinguish
        let mut dp = vec![1; MAX_TIME + 1];
        let mut do_cnt_rocks = vec![0; MAX_TIME + 1];
        for time in 1..=MAX_TIME {
            for cnt_rocks in 1..=MAX_ROCKS {
                let need_time = per_rock * cnt_rocks + per_round;
                if need_time > time {
                    break;
                }
                let left_time = time - need_time;
                let max_subsegment = dp[left_time];
                let max_value = max_subsegment * (cnt_rocks + 1);
                if max_value > dp[time] {
                    do_cnt_rocks[time] = cnt_rocks;
                    dp[time] = dp[time].max(max_value);
                }
            }
        }
        let res = binary_search_first_true(0..MAX_TIME, |time| dp[time] >= n);
        assert!(dp[res] >= n);
        out.println(res);
        let cnt_rocks = do_cnt_rocks[res];
        let segment_len = n.div_ceil(cnt_rocks + 1);
        let mut positions = vec![];
        for i in 0..cnt_rocks {
            let x = (i + 1) * segment_len;
            assert!(x < n);
            positions.push(x);
        }
        out.println(positions.len());
        out.println(positions);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "m";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    run_dragon(solve, "M", 5..6);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
