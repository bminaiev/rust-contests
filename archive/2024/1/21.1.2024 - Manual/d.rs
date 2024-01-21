//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let mut bad = vec![true; n];
        for _ in 0..k {
            let x = input.usize() - 1;
            bad[x] = false;
        }
        let order_y = input.vec::<usize>(n).sub_from_all(1);
        let mut in_y = vec![usize::MAX; n];
        for i in 0..n {
            in_y[order_y[i]] = i;
        }
        let mut dp = vec![usize::MAX / 2; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let mut any_bad = false;
            let cur_dp = dp[i];
            if bad[i] {
                dp[i + 1].update_min(cur_dp);
            }
            let mut seen = vec![false; n];
            let mut cnt_v = 0;
            let mut cnt_edges = 0;
            for j in i..n {
                if bad[j] {
                    any_bad = true;
                } else {
                    seen[j] = true;
                    let o_pos = in_y[j];
                    cnt_v += 1;
                    if o_pos > 0 && seen[order_y[o_pos - 1]] {
                        cnt_edges += 1;
                    }
                    if o_pos + 1 < n && seen[order_y[o_pos + 1]] {
                        cnt_edges += 1;
                    }
                }
                if !any_bad {
                    dp[j + 1].update_min(cur_dp + 1);
                }
                let cost_by_y = cur_dp + cnt_v - cnt_edges;
                dp[j + 1].update_min(cost_by_y);
            }
        }
        out.println(dp[n]);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
