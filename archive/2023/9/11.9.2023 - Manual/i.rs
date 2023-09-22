//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn ask(add: i64, input: &mut Input) -> i64 {
    out_line!("query", add);
    output().flush();
    input.i64()
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut mul = 1;
    let start = ask(0, input);
    let mut digits = vec![];
    for bit in 0..18 {
        let fail_add = binary_search_first_true(0..10, |add| {
            let ask_add = add * mul;
            let res = ask(ask_add, input);
            res != start + add
        });
        digits.push(10 - fail_add);
        if bit != 17 {
            mul *= 10;
        }
    }
    digits.reverse();
    let mut res = 0;
    for &d in digits.iter() {
        res = res * 10 + d;
    }
    out_line!("answer", res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
