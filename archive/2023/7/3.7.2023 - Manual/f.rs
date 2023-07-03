//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use std::cmp::{max, min};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let bi = input.usize();
    let bj = input.usize();
    let mn = min(bi, bj);
    let mx = max(bi, bj);
    out_line!("Yes");
    out_line!(n - 1);
    for i in 1..mn {
        out_line!(i, i, n - i, n - i);
    }
    for i in mx + 1..=n {
        let ln = (i - mn) as i32;
        out_line!(i, i, -ln, -ln);
    }
    let mx_len = if bj > bi { bj + 1 - bi } else { bi + 1 - bj };
    for ln in 2..=mx_len {
        if bj > bi {
            out_line!(mn + ln - 1, mx - ln + 1, -((ln - 1) as i32), ln - 1);
        } else {
            out_line!(mx - ln + 1, mn + ln - 1, ln - 1, -((ln - 1) as i32));
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
