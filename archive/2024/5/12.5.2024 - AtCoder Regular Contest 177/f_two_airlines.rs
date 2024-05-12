//{"name":"F - Two Airlines","group":"AtCoder - AtCoder Regular Contest 177","url":"https://atcoder.jp/contests/arc177/tasks/arc177_f","interactive":false,"timeLimit":5000,"tests":[{"input":"4 3\nAAJJ\n3 A\n1 J\n1 J\n","output":"2\n"},{"input":"8 3\nJJAAJJAJ\n2 A\n6 A\n8 J\n","output":"6\n"},{"input":"8 6\nJJAAJJAJ\n2 A\n6 A\n8 J\n8 J\n8 J\n8 J\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTwoAirlines"}}}

use std::vec;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let len = input.usize();
    let n = input.usize();
    let line = input
        .string()
        .iter()
        .map(|&c| if c == b'A' { 0 } else { 1 })
        .collect::<Vec<_>>();
    let mut line_pref_sum = vec![0];
    for x in line.iter() {
        let next_pref_sum = line_pref_sum.last().unwrap() + x;
        line_pref_sum.push(next_pref_sum);
    }
    let mut pos = vec![vec![]; 2];
    for _ in 0..n {
        let cur_pos = input.usize();
        let cur_type = if input.string()[0] == b'A' { 0 } else { 1 };
        pos[cur_type].push(cur_pos);
    }
    let get_cost = |cur_type: usize, idx: usize, need_pos: usize| -> i32 {
        if idx >= pos[cur_type].len() {
            return i32::MAX / 3;
        }
        let start_pos = pos[cur_type][idx];
        if need_pos <= start_pos {
            let len = start_pos - need_pos;
            let ones = line_pref_sum[start_pos] - line_pref_sum[need_pos];
            if cur_type == 0 {
                ones as i32
            } else {
                (len - ones) as i32
            }
        } else {
            let len = need_pos - start_pos;
            let ones = line_pref_sum[need_pos] - line_pref_sum[start_pos];
            if cur_type == 0 {
                (len - ones) as i32
            } else {
                ones as i32
            }
        }
    };
    let mut res = i32::MAX;
    let mut starts = vec![vec![0; len + 1]; 2];
    for cur_type in 0..2 {
        for now_pos in 0..=len {
            let mut idx =
                binary_search_first_true(0..pos[cur_type].len(), |i| pos[cur_type][i] >= now_pos);
            idx = idx.saturating_sub(1);
            starts[cur_type][now_pos] = idx;
        }
    }
    const M: usize = 20;
    for (next_0, next_1) in [(M, M), (3, M * M), (M * M, 3)].into_iter() {
        let mut dp = vec![Array2D::new(i32::MAX / 3, next_0 + 1, next_1 + 1); 2];
        for cur_type in 0..2 {
            let mut si = 0;
            let mut sj = 0;
            if cur_type == 0 {
                si += 1;
            } else {
                sj += 1;
            }
            dp[cur_type][si][sj] = get_cost(cur_type, 0, 0);
        }
        for pos in 0..len {
            let mut ndp = dp.clone();
            for cur_type in 0..2 {
                for i in 0..dp.len() {
                    for j in 0..dp[i].len() {
                        let now = ndp[cur_type][i][j];
                        if i + 1 < dp.len() {
                            ndp[cur_type][i + 1][j] = ndp[cur_type][i + 1][j].min(now);
                        }
                        if j + 1 < dp[i].len() {
                            ndp[cur_type][i][j + 1] = ndp[cur_type][i][j + 1].min(now);
                        }
                    }
                }
                for i in 0..dp.len() {
                    for j in 0..dp[i].len() {
                        let idx = starts[cur_type][pos] + (if cur_type == 0 { i } else { j });
                        dp[cur_type][i][j] = dp[cur_type][i][j]
                            .min(ndp[cur_type][i][j] + get_cost(cur_type, idx, pos));
                    }
                }
            }
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f_two_airlines";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
