//{"name":"A - Horizon","group":"AtCoder - Denso Create Programming Contest 2022(AtCoder Beginner Contest 239)","url":"https://atcoder.jp/contests/abc239/tasks/abc239_a","interactive":false,"timeLimit":2000,"tests":[{"input":"333\n","output":"65287.907678222\n"},{"input":"634\n","output":"90086.635834623\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AHorizon"}}}

use algo_lib::f;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let x = input.f64();
    let res = x * (f!(12800000.0) + x);
    out_line!(res.sqrt());
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
}
//END MAIN
