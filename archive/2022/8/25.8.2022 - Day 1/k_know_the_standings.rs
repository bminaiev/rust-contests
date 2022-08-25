//{"name":"K. Know The Standings","group":"Yandex - Day 1","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39546/problems/K/","interactive":false,"timeLimit":3000,"tests":[{"input":"4 42\n10 10 0.75\n10 10 0.75\n10 12 1\n10 12 1\n","output":"0.45625\n0.45625\n0.296875\n0.296875\n"},{"input":"4 42\n10 12 0.75\n10 12 0.75\n10 10 1\n10 10 1\n","output":"0.203125\n0.203125\n0.683238636363636\n0.683238636363636\n"},{"input":"5 100\n40 60 0.6\n40 61 1\n10 40 0.3\n10 40 0.4\n10 40 0.5\n","output":"0.12\n0\n0.112628571428571\n0.159739682539683\n0.206444444444444\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KKnowTheStandings"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Problem {
    reading: usize,
    coding: usize,
    pr_solve: f64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let max_time = input.usize();
    let problems = gen_vec(n, |_| Problem {
        reading: input.read(),
        coding: input.read(),
        pr_solve: input.f64().0,
    });
    let mut choosing = Array2D::new(0.0, max_time + 1, 1 << n);
    let mut solved = Array2D::new(0.0, max_time + 1, n);
    choosing[0][0] = 1.0;
    for time in 0..max_time {
        for i in 0..n {
            solved[time + 1][i] += solved[time][i];
        }

        for mask in 0..(1 << n) - 1 {
            let w = choosing[time][mask];
            if w == 0.0 {
                continue;
            }

            let mut sum_probs = 0.0;
            for i in 0..n {
                if ((1 << i) & mask) == 0 {
                    sum_probs += solved[time][i];
                }
            }
            let mut cnt = 0;
            for i in 0..n {
                if ((1 << i) & mask) == 0 {
                    cnt += 1;
                }
            }

            for i in 0..n {
                if ((1 << i) & mask) == 0 {
                    let next_pr = if sum_probs == 0.0 {
                        w / (cnt as f64)
                    } else {
                        w * solved[time][i] / sum_probs
                    };
                    let nmask = mask | (1 << i);
                    let solved_pr = next_pr * problems[i].pr_solve;
                    let need_time = time + problems[i].reading + problems[i].coding;
                    if need_time <= max_time {
                        choosing[need_time][nmask] += solved_pr;
                        solved[need_time][i] += solved_pr;
                    }
                    let not_solved_pr = next_pr * (1.0 - problems[i].pr_solve);
                    let need_time = time + problems[i].reading;
                    if need_time <= max_time {
                        choosing[need_time][nmask] += not_solved_pr;
                    }
                }
            }
        }
    }
    for i in 0..n {
        out_line!(solved[max_time][i]);
    }
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
