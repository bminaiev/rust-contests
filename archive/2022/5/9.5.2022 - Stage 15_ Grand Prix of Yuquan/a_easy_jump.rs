//{"name":"A. Easy Jump","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/A/","interactive":false,"timeLimit":1000,"tests":[{"input":"1 2 0\n50\n0\n1 2\n","output":"4.000000000000\n"},{"input":"2 3 1\n50 50\n1 1\n1 3\n","output":"6.000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AEasyJump"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_test(
    n: usize,
    max_h: usize,
    max_s: usize,
    probs: &[f64],
    can_restore: &[bool],
    t1: f64,
    t2: f64,
) -> f64 {
    const M: f64 = 1e11;
    let mut res = vec![Array2D::new(M, max_h + 1, max_s + 1); n + 1];
    for h in 0..=max_h {
        for s in 0..=max_s {
            res[n][h][s] = 0.0;
        }
    }
    const ITERS: usize = 3500;
    for round in (0..n).rev() {
        let p = probs[round];
        if can_restore[round] {
            let mut dp = vec![M; max_h + 1];
            for _ in 0..6000 {
                for h in 0..=max_h {
                    if h > 1 {
                        let nval = 1.0 + p * res[round + 1][h][max_s] + (1.0 - p) * (dp[h - 1]);
                        if nval < dp[h] {
                            dp[h] = nval;
                        }
                    }
                    if h > 1 {
                        let nval = 1.0 + p * res[round + 1][h][max_s] + (1.0 - p) * (t2 + dp[h]);
                        if nval < dp[h] {
                            dp[h] = nval;
                        }
                    }
                    if h < max_h && max_s != 0 {
                        let nval = t1 + dp[h + 1];
                        if nval < dp[h] {
                            dp[h] = nval;
                        }
                    }
                }
            }
            for h in 0..=max_h {
                for s in 0..=max_s {
                    res[round][h][s] = dp[h];
                }
            }
        } else {
            let next = res[round + 1].clone();
            let cur = &mut res[round];

            for s in 0..=max_s {
                for _ in 0..ITERS {
                    for h in 1..=max_h {
                        let mut new_val = cur[h][s];

                        if h > 1 {
                            let nval = 1.0 + p * next[h][s] + (1.0 - p) * (t2 + cur[h][s]);
                            if nval < new_val {
                                new_val = nval;
                            }
                        }
                        cur[h][s] = new_val;
                    }
                }
                for h in 1..=max_h {
                    let mut new_val = cur[h][s];
                    if h > 1 {
                        let nval = 1.0 + p * next[h][s] + (1.0 - p) * (cur[h - 1][s]);
                        if nval < new_val {
                            new_val = nval;
                        }
                    }
                    if h < max_h && s > 0 {
                        let nval = t1 + cur[h + 1][s - 1];
                        if nval < new_val {
                            new_val = nval;
                        }
                    }
                    cur[h][s] = new_val;
                }
            }
        }
    }
    let res = res[0][max_h][max_s];
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let max_h = input.usize();
    let max_s = input.usize();
    let probs = gen_vec(n, |_| input.f64().0 / 100.0);
    let mut can_restore = vec![false; n];
    let cnt = input.usize();
    for _ in 0..cnt {
        let pos = input.usize() - 1;
        can_restore[pos] = true;
    }
    let t1 = input.f64().0;
    let t2 = input.f64().0;

    let res = solve_test(n, max_h, max_s, &probs, &can_restore, t1, t2);
    out_line!(res);
}

fn stress() {
    let n = 1000;
    let max_h = 9;
    let max_s = 6;
    let mut rnd = Random::new(787788);
    let probs = gen_vec(n, |_| rnd.gen_in_range(1..100i32) as f64 / 100.0);
    let can_restore = vec![false; n];
    let t1 = 27.0;
    let t2 = 95.0;
    let res = solve_test(n, max_h, max_s, &probs, &can_restore, t1, t2);
    dbg!(res);
    let diff = (res - 426280.4788683343).abs() / res;
    assert!(diff < 1e-6);
}

fn solve123(input: &mut Input, _test_case: usize) {
    stress();
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
