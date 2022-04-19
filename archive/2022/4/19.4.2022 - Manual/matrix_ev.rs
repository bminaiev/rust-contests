//{"name":"matrix_ev","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"matrix_ev"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_case(days: usize, max_num: usize) -> f64 {
    let mut res = 0.0;
    let mut prob_min = vec![0.0; max_num + 1];
    prob_min[max_num] = 1.0;
    for day in 0..days {
        let mut new_prob_min = vec![0.0; max_num + 1];
        for i in 0..=max_num {
            let nprob = prob_min[i] / (max_num + 1) as f64;
            for j in 0..=max_num {
                if j <= i {
                    res += nprob * (j as f64);
                }
                let new_min = min(i, j);
                new_prob_min[new_min] += nprob;
            }
        }
        prob_min = new_prob_min;
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    for max_num in 1..=10 {
        for days in 1..=25 {
            let res = solve_case(days, max_num);
            out!(format!("{:.03} ", res));
        }
        out_line!();
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
}
//END MAIN
