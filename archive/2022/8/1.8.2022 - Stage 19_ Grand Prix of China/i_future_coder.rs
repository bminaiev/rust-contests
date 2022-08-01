//{"name":"I. Future Coder","group":"Yandex - Stage 19: Grand Prix of China","url":"https://official.contest.yandex.com/opencupXXII/contest/39025/problems/I/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n8\n3 -1 4 1 -5 9 2 -6\n1\n0\n","output":"19\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IFutureCoder"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut res = 0i64;
    let mut tot = 0;
    let mut leq1 = 0;
    let mut beq1 = 0;
    let mut b0 = 0;
    for v in a.into_iter() {
        if v == 0 {
            res += b0;
        } else if v == 1 {
            res += tot;
        } else if v > 1 {
            res += leq1;
        } else {
            res += beq1;
        }
        tot += 1;
        if v <= 1 {
            leq1 += 1;
        }
        if v >= 1 {
            beq1 += 1;
        }
        if v > 0 {
            b0 += 1;
        }
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
    // tester::run_stress(stress);
}
//END MAIN
