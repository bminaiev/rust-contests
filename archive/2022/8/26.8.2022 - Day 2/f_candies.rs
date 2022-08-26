//{"name":"F. Candies","group":"Yandex - Day 2","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39547/problems/F/","interactive":false,"timeLimit":1000,"tests":[{"input":"6 5\n1 1 4 5 1 4\n","output":"2\n"},{"input":"10 5\n1 2 5 2 1 2 3 4 8 4\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FCandies"}}}

use std::cmp::min;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let x = input.i32();
    let mut stack = vec![];
    let mut res = 0;
    for _ in 0..n {
        let val = input.i32();
        let val = min(val, x - val);
        if !stack.is_empty() && *stack.last_exn() == val {
            res += 1;
            stack.pop();
            continue;
        }
        stack.push(val);
    }
    if !stack.is_empty() {
        let mut i = 0;
        let mut j = stack.len() - 1;
        while i < j && stack[i] == stack[j] {
            i += 1;
            j -= 1;
            res += 1;
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
