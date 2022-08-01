//{"name":"B - AB Game","group":"AtCoder - AtCoder Regular Contest 145","url":"https://atcoder.jp/contests/arc145/tasks/arc145_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2 1\n","output":"2\n"},{"input":"27182818284 59045 23356\n","output":"10752495144\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BABGame"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.i64();
    let a = input.i64();
    let b = input.i64();
    let res = if a <= b {
        if n >= a {
            n - a + 1
        } else {
            0
        }
    } else {
        if n < a {
            0
        } else {
            let cnt = n / a;
            let res = (cnt - 1) * b;
            let more = min(b, n - cnt * a + 1);
            res + more
        }
    };
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
