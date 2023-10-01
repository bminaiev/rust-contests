//{"name":"B. Aleksa and Stack","group":"Codeforces - Codeforces Round 900 (Div. 3)","url":"https://codeforces.com/contest/1878/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n6\n7\n","output":"6 8 12\n7 11 14 20 22 100\n9 15 18 27 36 90 120\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAleksaAndStack"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |id| id + 7);
    for i in 0..(n - 2) {
        assert!(a[i + 2] * 3 % (a[i] + a[i + 1]) != 0);
    }
    out_line!(a);
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
