//{"name":"B. Декременты массива","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n3 5 4 1\n1 3 2 0\n3\n1 2 1\n0 1 0\n4\n5 3 7 2\n1 1 1 1\n5\n1 2 3 4 5\n1 2 3 4 6\n1\n8\n0\n1\n4\n6\n","output":"YES\nYES\nNO\nNO\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDekrementiMassiva"}}}

use std::cmp::max;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    let b = input.vec::<i32>(n);
    let mut deltas = vec![];
    let mut at_least = 0;
    for i in 0..n {
        if b[i] != 0 {
            deltas.push(a[i] - b[i]);
        } else {
            at_least.update_max(a[i] - b[i]);
        }
    }
    let mut sub = at_least;
    if !deltas.is_empty() {
        sub = deltas[0];
    }
    for i in 0..n {
        let r = a[i] - sub;
        a[i] = max(0, r);
    }
    out_line!(if a == b && sub >= 0 { "YES" } else { "NO" });
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
