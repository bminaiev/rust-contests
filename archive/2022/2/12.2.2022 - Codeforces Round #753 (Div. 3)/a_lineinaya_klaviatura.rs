//{"name":"A. Линейная клавиатура","group":"Codeforces - Codeforces Round #753 (Div. 3)","url":"https://codeforces.com/contest/1607/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nabcdefghijklmnopqrstuvwxyz\nhello\nabcdefghijklmnopqrstuvwxyz\ni\nabcdefghijklmnopqrstuvwxyz\ncodeforces\nqwertyuiopasdfghjklzxcvbnm\nqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\nqwertyuiopasdfghjklzxcvbnm\nabacaba\n","output":"13\n0\n68\n0\n74\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALineinayaKlaviatura"}}}

use std::ops::Index;

use algo_lib::collections::index_of::IndexOf;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let keyboard = input.string();
    let s = input.string();
    let mut res = 0;
    for w in s.windows(2) {
        let delta = (keyboard.index_of(&w[0]).unwrap() as i64
            - keyboard.index_of(&w[1]).unwrap() as i64)
            .abs();
        res += delta;
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
