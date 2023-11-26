//{"name":"B. AB-обмен","group":"Codeforces - CodeTON Round 7 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1896/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\nAB\n4\nBBBA\n4\nAABB\n","output":"1\n0\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BABObmen"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut first_a = n;
    let mut last_b = n;
    for i in 0..n {
        if s[i] == b'A' {
            if first_a == n {
                first_a = i;
            }
        } else {
            last_b = i;
        }
    }
    if last_b != n && first_a != n && last_b > first_a {
        out_line!(last_b - first_a);
    } else {
        out_line!(0);
    }
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
