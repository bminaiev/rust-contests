//{"name":"B - Count 1's","group":"AtCoder - AtCoder Regular Contest 137","url":"https://atcoder.jp/contests/arc137/tasks/arc137_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n0 1 1 0\n","output":"4\n"},{"input":"5\n0 0 0 0 0\n","output":"6\n"},{"input":"6\n0 1 0 1 0 1\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCount1s"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn max_add(a: &[i32]) -> i32 {
    let mut balance = 0;
    let mut smallest_balance = 0;
    let mut res = 0;
    for &x in a.iter() {
        balance += x * 2 - 1;
        smallest_balance.update_min(balance);
        res.update_max(balance - smallest_balance);
    }
    res
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    let max = max_add(&a);
    for x in a.iter_mut() {
        *x = 1 - *x;
    }
    let max2 = max_add(&a);
    out_line!(max + max2 + 1);
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
