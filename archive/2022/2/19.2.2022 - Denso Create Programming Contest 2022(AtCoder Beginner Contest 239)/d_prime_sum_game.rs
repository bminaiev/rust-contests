//{"name":"D - Prime Sum Game","group":"AtCoder - Denso Create Programming Contest 2022(AtCoder Beginner Contest 239)","url":"https://atcoder.jp/contests/abc239/tasks/abc239_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 3 4\n","output":"Aoki\n"},{"input":"1 100 50 60\n","output":"Takahashi\n"},{"input":"3 14 1 5\n","output":"Aoki\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrimeSumGame"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::primes::gen_primes_table;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let (a, b, c, d) = input.read();
    let is_prime = gen_primes_table(300);
    let takahashi_wins =
        (a..=b).any(|x: usize| -> bool { !(c..=d).any(|y: usize| is_prime[x + y]) });
    if takahashi_wins {
        out_line!("Takahashi");
    } else {
        out_line!("Aoki");
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
