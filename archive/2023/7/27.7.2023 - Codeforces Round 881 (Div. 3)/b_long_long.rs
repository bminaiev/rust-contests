//{"name":"B. Лонг лонг","group":"Codeforces - Codeforces Round 881 (Div. 3)","url":"https://codeforces.com/contest/1843/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6\n-1 7 -4 -2 5 -8\n8\n-1 0 0 -2 1 0 -3 0\n5\n2 -1 0 -3 -7\n5\n0 -17 0 1 0\n4\n-1 0 -2 -1\n","output":"27 3\n7 2\n13 1\n18 1\n4 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLongLong"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    let a = a.into_iter().filter(|&x| x != 0).collect::<Vec<_>>();
    let mut max_sum = 0i32;
    let mut ops = 1;
    if a.is_empty() {
        out_line!(0, 0);
        return;
    }
    max_sum += a[0].abs();
    for i in 1..a.len() {
        max_sum += a[i].abs();
        if (a[i] > 0) != (a[i - 1] > 0) {
            ops += 1;
        }
    }
    if a[0] > 0 {
        ops -= 1;
    }
    if a[a.len() - 1] > 0 {
        ops -= 1;
    }
    out_line!(max_sum, (ops + 1) / 2);
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
