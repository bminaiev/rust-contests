//{"name":"B. Longest Increasing Subsequence","group":"Yandex - Stage 18: Grand Prix of Bytedance","url":"https://official.contest.yandex.com/opencupXXII/contest/39023/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n7 41 53 58 75 78 81\n","output":"22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLongestIncreasingSubsequence"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<u64>(n);
    const MAX_LVL: usize = 62;
    let mut max_in_lvl = vec![1; MAX_LVL];
    for i in 0..MAX_LVL - 1 {
        max_in_lvl[i + 1] = 1u64 << i;
    }
    let mut dp = vec![0u64; MAX_LVL];
    for w in a.windows(2) {
        let len = w[1] - w[0];
        let mut ndp = dp.clone();
        let mut on_lvl = max_in_lvl.clone();
        let mut more = len;
        for i in 0..on_lvl.len() {
            on_lvl[i].update_min(more);
            more -= on_lvl[i];
        }
        // dbg!(len, on_lvl, dp)e
        let mut cur_max_dp = 0;
        for lvl in 0..MAX_LVL {
            cur_max_dp.update_max(dp[lvl]);
            ndp[lvl].update_max(cur_max_dp + on_lvl[lvl]);
            if lvl == 0 {
                cur_max_dp += 1;
            } else if on_lvl[lvl] != 0 && *on_lvl.get(lvl + 2).unwrap_or(&0) == 0 {
                if *on_lvl.get(lvl + 1).unwrap_or(&0) >= 1 {
                    ndp[lvl + 1].update_max(cur_max_dp + on_lvl[lvl] + 1);
                }

                if !len.is_power_of_two() {
                    cur_max_dp += 1;
                }
            }
        }
        dp = ndp;
    }
    dp[0] += 1;
    let res = *dp.iter().max().unwrap();
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
