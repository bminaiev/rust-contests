//{"name":"C. Устранение минимума","group":"Codeforces - Codeforces Round #753 (Div. 3)","url":"https://codeforces.com/contest/1607/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1\n10\n2\n0 0\n3\n-1 2 0\n4\n2 10 1 7\n2\n2 3\n5\n3 2 -4 -2 0\n2\n-1 1\n1\n-2\n","output":"10\n0\n2\n5\n2\n2\n2\n-2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUstranenieMinimuma"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i64>(n).sorted();
    let mut res = a[0];
    for w in a.windows(2) {
        res.update_max(w[1] - w[0]);
    }
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
