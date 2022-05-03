//{"name":"A. Stickers for BSUIR Open","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2 3\n","output":"1.0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AStickersForBSUIROpen"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search_float::float_binary_search_first_true;
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.f64();
    let m = input.f64();
    let k = input.i64();

    let res =
        float_binary_search_first_true(OrdF64(0.0), OrdF64(2000.0), 100, |check_size| -> bool {
            let sz1 = (n / check_size).0 as i64;
            let sz2 = (m / check_size).0 as i64;
            sz1 * sz2 < k
        });
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
}
//END MAIN
