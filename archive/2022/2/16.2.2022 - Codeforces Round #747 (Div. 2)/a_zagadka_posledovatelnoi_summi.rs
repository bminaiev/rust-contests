//{"name":"A. Загадка последовательной суммы","group":"Codeforces - Codeforces Round #747 (Div. 2)","url":"https://codeforces.com/contest/1594/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n2\n3\n6\n100\n25\n3000000000000\n","output":"0 1\n-1 2\n1 2\n1 3\n18 22\n-2 7\n999999999999 1000000000001\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZagadkaPosledovatelnoiSummi"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let need = input.i64();
    out_line!((-need + 1), need);
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
