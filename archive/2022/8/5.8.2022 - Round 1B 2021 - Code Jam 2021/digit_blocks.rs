//{"name":"Digit Blocks","group":"Google Coding Competitions - Round 1B 2021 - Code Jam 2021","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000435baf/00000000007ae37b","interactive":false,"timeLimit":60000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DigitBlocks"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, n: usize, b: usize, p: i64) {
    let mut already = vec![0; n];
    for _ in 0..n * b {
        let value = input.i64();
        let mut pos = 0;
        if value == 9 {
            while already[pos] == b {
                pos += 1;
            }
        } else {
            for &at_most in [b - 2, b - 1, b].iter() {
                pos = 0;
                while pos != already.len() && already[pos] > at_most {
                    pos += 1;
                }
                if pos != already.len() {
                    break;
                }
            }
        }
        assert!(pos < already.len());

        already[pos] += 1;
        out_line!(pos + 1);
        output().flush();
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    let n = input.usize();
    let b = input.usize();
    let p = input.i64();
    dbg!("expected sum", p);
    for i in 0usize..t {
        solve(&mut input, n, b, p);
    }
    input.i32();
    // assert_eq!(input.i32(), 1);
    output().flush();
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
