//{"name":"Different Medians","group":"CodeChef - START51","url":"https://www.codechef.com/problems-old/DIFFMED","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n3\n","output":"2 1\n2 1 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DifferentMedians"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = vec![0i32];
    while a.len() < n {
        if a.len() % 2 == 1 {
            let mn = a.iter().min().unwrap();
            a.push(mn - 1);
        } else {
            let mx = a.iter().max().unwrap();
            a.push(mx + 1);
        }
    }
    let mn = a.iter().min().unwrap();
    let a: Vec<_> = a.iter().map(|x| *x - mn + 1).collect();
    out_line!(a);
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
