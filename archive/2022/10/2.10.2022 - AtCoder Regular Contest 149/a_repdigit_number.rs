//{"name":"A - Repdigit Number","group":"AtCoder - AtCoder Regular Contest 149","url":"https://atcoder.jp/contests/arc149/tasks/arc149_a","interactive":false,"timeLimit":2000,"tests":[{"input":"7 12\n","output":"888888\n"},{"input":"9 12\n","output":"888888888\n"},{"input":"1 3\n","output":"9\n"},{"input":"1000 25\n","output":"-1\n"},{"input":"30 1\n","output":"999999999999999999999999999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARepdigitNumber"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.i64();
    let mut ways = vec![];
    for digit in 1..=9 {
        let mut rem = 0;
        for cnt in 1..=n {
            rem = (rem * 10 + digit) % m;
            if rem == 0 {
                ways.push((cnt, digit));
            }
        }
    }
    match ways.iter().max() {
        None => out_line!(-1),
        Some(&(cnt, digit)) => {
            for _ in 0..cnt {
                out!(digit);
            }
            out_line!()
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
