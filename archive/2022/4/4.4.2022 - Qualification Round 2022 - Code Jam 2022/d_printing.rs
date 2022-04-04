//{"name":"3D Printing","group":"Google Coding Competitions - Qualification Round 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a4672b","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n300000 200000 300000 500000\n300000 200000 500000 300000\n300000 500000 300000 200000\n1000000 1000000 0 0\n0 1000000 1000000 1000000\n999999 999999 999999 999999\n768763 148041 178147 984173\n699508 515362 534729 714381\n949704 625054 946212 951187\n","output":"Case #1: 300000 200000 300000 200000\nCase #2: IMPOSSIBLE\nCase #3: 400001 100002 100003 399994\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrinting"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let n = 3;
    let m = 4;
    let a = input.matrix::<i32>(n, m);
    let mut colors = vec![std::i32::MAX; m];
    for i in 0..n {
        for j in 0..m {
            colors[j].update_min(a[i][j]);
        }
    }
    let cur_sum = colors.iter().sum::<i32>();
    out!(format!("Case #{}: ", test_case));
    const NEED: i32 = 1_000_000;
    if cur_sum < NEED {
        out_line!("IMPOSSIBLE");
    } else {
        let mut more = cur_sum - NEED;
        for x in colors.iter_mut() {
            let remove_here = min(*x, more);
            more -= remove_here;
            *x -= remove_here;
        }
        out_line!(colors);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
