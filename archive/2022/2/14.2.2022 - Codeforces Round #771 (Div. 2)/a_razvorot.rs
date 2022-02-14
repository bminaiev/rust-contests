//{"name":"A. Разворот","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n1\n3\n2 1 3\n4\n1 4 2 3\n5\n1 2 3 4 5\n","output":"1\n1 2 3\n1 2 4 3\n1 2 3 4 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARazvorot"}}}

use algo_lib::collections::index_of::IndexOf;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut p = input.read_vec::<usize>(n);
    for pos in 0..n {
        if p[pos] != pos + 1 {
            let index_of_pos = p.index_of(&(pos + 1)).unwrap();
            p[pos..=index_of_pos].reverse();
            break;
        }
    }
    out_line!(p);
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
