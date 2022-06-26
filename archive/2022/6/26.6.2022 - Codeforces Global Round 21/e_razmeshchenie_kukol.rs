//{"name":"E. Размещение кукол","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2 2 0\n","output":"5\n"},{"input":"10\n12 11 8 8 6 6 6 5 3 2 1\n","output":"2596\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERazmeshchenieKukol"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod7;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize() + 1;
    let a = input.vec::<usize>(n);
    let c = CombinationsFact::<Mod>::new(400_000 + 10);
    let mut res = Mod::ZERO;
    for x in 0..n {
        let y = a[x];
        // for last_y in 0..y {
        // res += c.c(x + last_y, x);
        // }
        res += c.c(x + y, x + 1);
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    // tester::run_stress(stress);
}
//END MAIN
