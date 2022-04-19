//{"name":"Double or One Thing","group":"Google Coding Competitions - Round 1A 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000877ba5/0000000000aa8e9c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\nPEEL\nAAAAAAAAAA\nCODEJAMDAY\n","output":"Case #1: PEEEEL\nCase #2: AAAAAAAAAA\nCase #3: CCODDEEJAAMDAAY\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DoubleOrOneThing"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let s = input.string();
    let mut res = vec![];
    for i in 0..s.len() {
        let mut double = false;
        for j in i + 1..s.len() {
            if s[j] != s[i] {
                if s[j] > s[i] {
                    double = true;
                }
                break;
            }
        }
        res.push(s[i]);
        if double {
            res.push(s[i]);
        }
    }
    out_line!(format!("Case #{}: {}", test_case, vec2str(&res)));
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
