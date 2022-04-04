//{"name":"Punched Cards","group":"Google Coding Competitions - Qualification Round 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a4621b","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n3 4\n2 2\n2 3\n","output":"Case #1:\n..+-+-+-+\n..|.|.|.|\n+-+-+-+-+\n|.|.|.|.|\n+-+-+-+-+\n|.|.|.|.|\n+-+-+-+-+\nCase #2:\n..+-+\n..|.|\n+-+-+\n|.|.|\n+-+-+\nCase #3:\n..+-+-+\n..|.|.|\n+-+-+-+\n|.|.|.|\n+-+-+-+\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PunchedCards"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut res = Array2D::new(b'.', n * 2 + 1, m * 2 + 1);
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            if i % 2 == 0 && j % 2 == 0 {
                res[i][j] = b'+';
            } else if i % 2 == 0 && j % 2 == 1 {
                res[i][j] = b'-';
            } else if i % 2 == 1 && j % 2 == 0 {
                res[i][j] = b'|';
            }
        }
    }
    res[0][0] = b'.';
    res[0][1] = b'.';
    res[1][0] = b'.';
    out_line!(format!("Case #{}:", test_case));
    for i in 0..res.len() {
        out_line!(vec2str(&res[i]));
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
