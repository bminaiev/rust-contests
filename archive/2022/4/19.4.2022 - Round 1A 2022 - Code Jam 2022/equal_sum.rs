//{"name":"Equal Sum","group":"Google Coding Competitions - Round 1A 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000877ba5/0000000000aa8fc1","interactive":false,"timeLimit":5000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EqualSum"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let mut res = vec![];
    let mut x = 1;
    const MAX: i64 = 1e9 as i64;
    while x < MAX {
        res.push(x);
        x *= 2;
    }
    x /= 2;
    while res.len() < n {
        x -= 1;
        res.push(x);
    }
    out_line!(res);
    output().flush();
    for _ in 0..n {
        res.push(input.i64());
    }
    let sum = res.iter().sum::<i64>();
    assert_eq!(sum % 2, 0);
    let mut need_more = sum / 2;
    res.sort();
    res.reverse();
    let mut chosen = vec![];
    for &x in res.iter() {
        if need_more >= x {
            need_more -= x;
            chosen.push(x);
        }
    }
    assert_eq!(need_more, 0);
    out_line!(chosen);
    output().flush();
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    // input.skip_whitespace();
    // input.peek().is_none()
    true
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
    // tester::run_tests();
    tester::run_locally();
    // tester::run_single_test("1");
}
//END MAIN
