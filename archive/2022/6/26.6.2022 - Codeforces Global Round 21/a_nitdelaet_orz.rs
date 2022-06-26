//{"name":"A. НИТ делает orz","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 3\n3 4\n5 5\n0 2 4 6 8\n1 9\n10\n5 7\n7 15 30 29 27\n3 39548743\n10293834 10284344 13635445\n","output":"7\n13\n11\n31\n48234367\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANITDelaetOrz"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let z = input.i32();
    let a = input.vec::<i32>(n);
    let res = a.into_iter().map(|x| x | z).max().unwrap();
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
    // tester::run_stress(stress);
}
//END MAIN
