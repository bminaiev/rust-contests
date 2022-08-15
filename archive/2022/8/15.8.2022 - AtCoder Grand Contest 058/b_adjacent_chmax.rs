//{"name":"B - Adjacent Chmax","group":"AtCoder - AtCoder Grand Contest 058","url":"https://atcoder.jp/contests/agc058/tasks/agc058_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3 2\n","output":"4\n"},{"input":"4\n2 1 3 4\n","output":"11\n"},{"input":"10\n4 9 6 3 8 10 1 2 7 5\n","output":"855\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAdjacentChmax"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut pos = vec![0; n];
    for p in 0..n {
        pos[a[p]] = p;
    }
    let mut mx = Array2D::new(0, n, n);
    for i in 0..n {
        for j in i..n {
            mx[i][j] = a[j];
            if j > i {
                let cur = mx[i][j - 1];
                mx[i][j].update_max(cur);
            }
            mx[j][i] = mx[i][j];
        }
    }
    let mut dp = Array2D::new(Mod::ZERO, n, n);
    for start in 0..n {
        let p = pos[start];
        let max = mx[0][p];
        if max == start {
            dp[0][start] = Mod::ONE;
        }
    }
    for i in 0..(n - 1) {
        let mut fenw_more = Fenwick::new(n);

        for next in (0..n).rev() {
            let p = pos[next];
            if p <= i {
                fenw_more.add(p, dp[i][next]);
            }
            if mx[i + 1][p] != next {
                continue;
            }
            if p > 0 {
                dp[i + 1][next] += fenw_more.get_sum(p - 1);
            }
        }

        for last in 0..n {
            let ways = dp[i][last];
            if ways == Mod::ZERO {
                continue;
            }
            for next in last..last + 1 {
                let p = pos[next];
                if mx[i + 1][p] != next {
                    continue;
                }
                dp[i + 1][next] += ways;
            }
        }

        let mut fenw_more = Fenwick::new(n);

        for next in (0..n) {
            let p = pos[next];
            // if p <= i {
            fenw_more.add(p, dp[i][next]);
            // }
            if mx[i + 1][p] != next {
                continue;
            }
            if p > 0 {
                dp[i + 1][next] += fenw_more.get_sum(p - 1);
            }
        }

        // for last in 0..n {
        //     let ways = dp[i][last];
        //     if ways == Mod::ZERO {
        //         continue;
        //     }
        //     let prev_pos = pos[last];
        //     for next in 0..n {
        //         let p = pos[next];
        //         if mx[i + 1][p] != next {
        //             continue;
        //         }
        //         if last >= next {
        //             continue;
        //         }
        //         if prev_pos > p {
        //             continue;
        //         }
        //         dp[i + 1][next] += ways;
        //     }
        // }
    }
    let mut res = Mod::ZERO;
    for last in 0..n {
        res += dp[n - 1][last];
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
