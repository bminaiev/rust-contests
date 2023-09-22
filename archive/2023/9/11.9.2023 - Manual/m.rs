//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut dp_pos = 0;
    let mut dp_neg = std::i64::MAX / 10;
    for &v in a.iter() {
        let mut ndp_pos = std::i64::MAX / 10;
        let mut ndp_neg = std::i64::MAX / 10;
        if (v > 0) {
            ndp_pos = min(dp_pos, dp_neg + v + 1);
            ndp_neg = min(dp_neg, dp_pos + v + 1);
        } else if (v == 0) {
            ndp_pos = min(dp_pos + 1, dp_neg + 1);
            ndp_neg = min(dp_pos + 1, dp_neg + 1);
        } else {
            let v = -v;
            ndp_pos = min(dp_neg, dp_pos + v + 1);
            ndp_neg = min(dp_pos, dp_neg + v + 1);
        }
        dp_pos = ndp_pos;
        dp_neg = ndp_neg;
    }
    out_line!(dp_pos);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
