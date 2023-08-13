//{"name":"A. Шифр шифер","group":"Codeforces - Codeforces Round 878 (Div. 3)","url":"https://codeforces.com/contest/1840/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n8\nabacabac\n5\nqzxcq\n20\nccooddeeffoorrcceess\n","output":"ac\nq\ncodeforces\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AShifrShifer"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let _len = input.usize();
    let s = input.string();
    let mut res = vec![];
    let mut i = 0;
    while i < s.len() {
        let mut j = i + 1;
        while s[j] != s[i] {
            j += 1;
        }
        res.push(s[i]);
        i = j + 1;
    }
    let s = String::from_utf8(res).unwrap();
    out_line!(s);
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
