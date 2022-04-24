//{"name":"A. Рубка бревен","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n2 4 2 1\n1\n1\n","output":"errorgorn\nmaomao90\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARubkaBreven"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut turns = 0i64;
    let a = input.vec::<i64>(n);
    for x in a.iter() {
        turns += x - 1;
    }
    if turns % 2 == 1 {
        out_line!("errorgorn");
    } else {
        out_line!("maomao90");
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
}
//END MAIN
