//{"name":"A. Длинное сравнение","group":"Codeforces - Educational Codeforces Round 118 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1613/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 1\n19 0\n10 2\n100 1\n1999 0\n2 3\n1 0\n1 0\n99 0\n1 2\n","output":">\n=\n<\n=\n<\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADlinnoeSravnenie"}}}

use std::cmp::{min, Ordering};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoSettings;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoType};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn cmp(mut x1: i64, mut p1: i64, mut x2: i64, mut p2: i64) -> Ordering {
    let sub = min(p1, p2);
    p1 -= sub;
    p2 -= sub;
    if p1 > 10 {
        return Ordering::Greater;
    }
    if p2 > 10 {
        return Ordering::Less;
    }
    x1 *= 10i64.pow(p1 as u32);
    x2 *= 10i64.pow(p2 as u32);

    x1.cmp(&x2)
}

fn solve(input: &mut Input, _test_case: usize) {
    let (x1, p1, x2, p2) = input.read();
    match cmp(x1, p1, x2, p2) {
        Ordering::Less => out_line!("<"),
        Ordering::Greater => out_line!(">"),
        Ordering::Equal => out_line!("="),
    }
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
