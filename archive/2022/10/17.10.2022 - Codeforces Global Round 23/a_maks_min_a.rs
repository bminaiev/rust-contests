//{"name":"A. МаксМинА","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n3 2\n0 1 0\n5 3\n1 0 1 1 0\n2 2\n1 1\n4 4\n0 0 0 0\n6 3\n0 0 1 0 0 1\n7 5\n1 1 1 1 1 1 1\n5 3\n0 0 1 0 0\n","output":"YES\nYES\nYES\nNO\nYES\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMaksMinA"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.vec::<usize>(n);
    if a.iter().all(|x| *x == 0) {
        out_line!("NO");
    } else {
        out_line!("YES");
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
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
