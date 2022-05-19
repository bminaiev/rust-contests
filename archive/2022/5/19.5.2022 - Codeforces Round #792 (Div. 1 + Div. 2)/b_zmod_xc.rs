//{"name":"B. Z mod X = C","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 3 4\n127 234 421\n2 7 8\n59 94 388\n","output":"12 11 4\n1063 234 1484\n25 23 8\n2221 94 2609\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZModXC"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.i64();
    let b = input.i64();
    let c = input.i64();
    let mut x = a;
    let y = b;
    let z = c;

    x += (1e9 as i64) * y;

    assert!(x % y == a);
    assert!(y % z == b);
    assert!(z % x == c);
    out_line!(x, y, z);
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
