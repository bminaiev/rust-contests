//{"name":"d1000000","group":"Google Coding Competitions - Qualification Round 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a46471","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n4\n6 10 12 8\n6\n5 4 5 4 4 4\n10\n10 10 7 6 7 4 4 5 7 4\n1\n10\n","output":"Case #1: 4\nCase #2: 5\nCase #3: 9\nCase #4: 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1000000"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sorted();
    let mut res = 0;
    for &x in a.iter() {
        if x >= res + 1 {
            res += 1;
        }
    }
    out_line!(format!("Case #{}: {}", test_case, res));
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
