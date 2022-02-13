//{"name":"A. Сортировка частей","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/0?locale=ru","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n2 2 1\n4\n3 1 2 1\n5\n1 2 2 4 4\n","output":"YES\nYES\nNO\n"}],"testType":"multiEof","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASortirovkaChastei"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i32>(n);
    if a == a.sorted() {
        out_line!("NO");
    } else {
        out_line!("YES");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let tc = input.usize();
    for t in 0..tc {
        solve(&mut input, t);
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
