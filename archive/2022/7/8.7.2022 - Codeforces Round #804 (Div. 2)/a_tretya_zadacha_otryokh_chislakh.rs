//{"name":"A. Третья задача о трёх числах","group":"Codeforces - Codeforces Round #804 (Div. 2)","url":"https://codeforces.com/contest/1699/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4\n1\n12\n2046\n194723326\n","output":"3 3 1\n-1\n2 4 6\n69 420 666\n12345678 87654321 100000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATretyaZadachaOTryokhChislakh"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    if n % 2 == 1 {
        out_line!(-1);
    } else {
        out_line!((n / 2) ^ 1, 1, 1);
    }
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
