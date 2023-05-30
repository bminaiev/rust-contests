//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn query(d: i32, input: &mut Input) -> i32 {
    out_line!("?", d);
    output().flush();
    input.i32()
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut shifts = vec![];
    let mut first_after_found = false;
    if query(0, input) == 1 {
        shifts.push(0);
        first_after_found = true;
    }
    let mut cur_shift = 0;
    while shifts.len() < 2 {
        let mut d = 1;
        if shifts.len() == 1 {
            d = 2;
        }
        if first_after_found {
            d = shifts[0] + 2;
            if d % 2 == 0 {
                d += 1;
            }
        }

        cur_shift += d;
        if query(d, input) == 1 {
            shifts.push(cur_shift);
            first_after_found = true;
        } else {
            first_after_found = false;
        }
    }
    let delta = shifts[1] - shifts[0];
    assert!(delta % 2 == 1);
    let first = (delta - 1) / 2;
    out_line!("!", first * first - shifts[0]);
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
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
