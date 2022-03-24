//{"name":"K. Kathmandu","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/K","interactive":false,"timeLimit":250,"tests":[{"input":"3 10 3\n2\n4\n7\n","output":"Y\n"},{"input":"4 10 3\n2\n4\n7\n","output":"N\n"},{"input":"5 5 0\n","output":"Y\n"},{"input":"4 8 2\n5\n7\n","output":"Y\n"},{"input":"4 8 2\n3\n4\n","output":"Y\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KKathmandu"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let need = input.i32();
    let total_time = input.i32();
    let n = input.usize();
    let mut times = input.vec::<i32>(n);
    times.push(0);
    times.push(total_time);
    times.sort();
    for w in times.windows(2) {
        if w[1] - w[0] >= need {
            out_line!("Y");
            return;
        }
    }
    out_line!("N");
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
}
//END MAIN
