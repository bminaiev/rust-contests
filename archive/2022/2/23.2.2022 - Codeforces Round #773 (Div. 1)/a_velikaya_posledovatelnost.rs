//{"name":"A. Великая последовательность","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 4\n1 16 4 4\n6 2\n1 2 2 2 4 7\n5 3\n5 2 3 5 15\n9 10\n10 10 10 20 1 100 200 2000 3\n","output":"0\n2\n3\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AVelikayaPosledovatelnost"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let x = input.i64();
    let mut a = input.vec::<i64>(n);
    a.sort();
    let mut used = vec![false; n];
    let mut it = 0;
    for i in 0..n {
        while it != i && (used[it] || a[it] * x < a[i]) {
            it += 1;
        }
        if a[it] * x == a[i] {
            used[it] = true;
            used[i] = true;
        }
    }
    let mut res = 0;
    for i in 0..n {
        if !used[i] {
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
