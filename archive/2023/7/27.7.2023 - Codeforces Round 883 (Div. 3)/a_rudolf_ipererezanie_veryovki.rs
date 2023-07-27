//{"name":"A. Рудольф и перерезание верёвки","group":"Codeforces - Codeforces Round 883 (Div. 3)","url":"https://codeforces.com/contest/1846/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n4 3\n3 1\n1 2\n4\n9 2\n5 2\n7 7\n3 4\n5\n11 7\n5 10\n12 9\n3 2\n1 5\n3\n5 6\n4 5\n7 7\n","output":"2\n2\n3\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARudolfIPererezanieVeryovki"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut res = 0;
    for _ in 0..n {
        let pos = input.i32();
        let len = input.i32();
        if pos - len > 0 {
            res += 1;
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
