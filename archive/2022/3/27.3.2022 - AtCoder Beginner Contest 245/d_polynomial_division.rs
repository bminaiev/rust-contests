//{"name":"D - Polynomial division","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_d","interactive":false,"timeLimit":2000,"tests":[{"input":"1 2\n2 1\n12 14 8 2\n","output":"6 4 2\n"},{"input":"1 1\n100 1\n10000 0 -1\n","output":"100 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPolynomialDivision"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let mut a = input.vec::<i64>(n + 1);
    a.reverse();
    let mut c = input.vec::<i64>(n + m + 1);
    c.reverse();
    let mut b = vec![];
    for i in 0..=m {
        let cur = c[i] / a[0];
        b.push(cur);
        for j in 0..=n {
            c[j + i] -= a[j] * cur;
        }
    }
    b.reverse();
    out_line!(b);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
