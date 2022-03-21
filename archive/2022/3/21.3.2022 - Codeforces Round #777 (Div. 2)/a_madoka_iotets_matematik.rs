//{"name":"A. Мадока и отец математик","group":"Codeforces - Codeforces Round #777 (Div. 2)","url":"http://codeforces.com/contest/1647/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n2\n3\n4\n5\n","output":"1\n2\n21\n121\n212\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMadokaIOtetsMatematik"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let need_sum = input.usize();
    for len in 1.. {
        for &first in [2, 1].iter() {
            let sum = (len / 2) * (3 - first) + (len + 1) / 2 * first;
            if sum == need_sum {
                for pos in 0..len {
                    if pos % 2 == 0 {
                        out!(first);
                    } else {
                        out!(3 - first);
                    }
                }
                out_line!();
                return;
            }
        }
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
