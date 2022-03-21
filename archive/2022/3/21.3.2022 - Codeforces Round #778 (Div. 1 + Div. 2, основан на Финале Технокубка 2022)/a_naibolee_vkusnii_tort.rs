//{"name":"A. Наиболее вкусный торт","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"http://codeforces.com/contest/1654/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6\n5 2 1 4 7 3\n3\n32 78 78\n3\n69 54 91\n8\n999021 999021 999021 999021 999652 999021 999021 999021\n2\n1000000000 1000000000\n","output":"12\n156\n160\n1998673\n2000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANaiboleeVkusniiTort"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i64>(n);
    a.sort();
    a.reverse();
    let res: i64 = a[0] + a[1];
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
