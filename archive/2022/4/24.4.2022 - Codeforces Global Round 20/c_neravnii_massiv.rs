//{"name":"C. Неравный массив","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n1 1 1 1 1\n5\n2 1 1 1 2\n6\n1 1 2 3 3 4\n6\n1 2 1 4 5 4\n","output":"2\n1\n2\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNeravniiMassiv"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    let mut first = n;
    let mut last = n;
    for i in 0..(n - 1) {
        if a[i] == a[i + 1] {
            last = i;
            if first == n {
                first = i;
            }
        }
    }
    let len = last - first;
    let res = if len == 0 {
        0
    } else if len <= 2 {
        1
    } else {
        len - 1
    };
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
