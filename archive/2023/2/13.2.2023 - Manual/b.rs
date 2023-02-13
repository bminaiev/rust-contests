//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut s = vec![];
    for _i in 0..n {
        s.push(input.string());
    }
    let res = RecursiveFunction3::new(|f, x: usize, y1: usize, y2: usize| -> i64 {
        for pos in y1..y2 {
            let c = s[x][pos];
            if c == b'.' {
                continue;
            }
            if c >= b'0' && c <= b'9' {
                return (c - b'0') as i64;
            }
            let lhs = f.call(x + 1, y1, pos);
            let rhs = f.call(x + 1, pos + 1, y2);
            if c == b'+' {
                return lhs + rhs;
            }
            if c == b'-' {
                return lhs - rhs;
            }
            if c == b'*' {
                return lhs * rhs;
            }
            assert!(false);
        }
        assert!(false);
        0
    })
    .call(0, 0, m);
    out_line!(res);
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
