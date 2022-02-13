//{"name":"B. MEX и массив","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 2\n3\n2 0 1\n4\n2 0 5 1\n5\n0 1 1 0 1\n","output":"4\n14\n26\n48\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMEXIMassiv"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i32>(n);
    let mut res = 0;
    for l in 0..n {
        for r in l + 1..=n {
            for x in l..r {
                res += 1;
                if a[x] == 0 {
                    res += 1;
                }
            }
        }
    }
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
