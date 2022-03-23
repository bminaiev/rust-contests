//{"name":"A - Coprime Pair","group":"AtCoder - AtCoder Regular Contest 137","url":"https://atcoder.jp/contests/arc137/tasks/arc137_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2 4\n","output":"1\n"},{"input":"14 21\n","output":"5\n"},{"input":"1 100\n","output":"99\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACoprimePair"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let l = input.i64();
    let r = input.i64();
    for dist in (0..=(r - l)).rev() {
        for x in l..=(r - dist) {
            let y = x + dist;
            if gcd(x, y) == 1 {
                out_line!(dist);
                return;
            }
        }
    }
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
