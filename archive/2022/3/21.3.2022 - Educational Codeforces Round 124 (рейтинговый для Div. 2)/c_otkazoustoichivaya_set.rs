//{"name":"C. Отказоустойчивая сеть","group":"Codeforces - Educational Codeforces Round 124 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1651/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3\n1 10 1\n20 4 25\n4\n1 1 1 1\n1000000000 1000000000 1000000000 1000000000\n","output":"31\n1999999998\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COtkazoustoichivayaSet"}}}

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
    let b = input.vec::<i64>(n);
    let res2 = (a[0] - b[0]).abs() + (a[n - 1] - b[n - 1]).abs();
    let res3 = (a[0] - b[n - 1]).abs() + (a[n - 1] - b[0]).abs();
    let res = min(res2, res3);
    out_line!(res);
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
