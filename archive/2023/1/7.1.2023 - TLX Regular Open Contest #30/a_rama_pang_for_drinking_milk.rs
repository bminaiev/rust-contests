//{"name":"A. rama_pang for drinking milk","group":"TLX - TLX Regular Open Contest #30","url":"https://tlx.toki.id/contests/troc-30/problems/A","interactive":false,"timeLimit":1000,"tests":[{"input":"19 6 4\n","output":"27\n"},{"input":"69 4 20\n","output":"69\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARamaPangForDrinkingMilk"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut res = 0;
    let n = input.i32();
    let x = input.i32();
    let y = input.i32();
    for packs in 0..=100 {
        let need = packs * y;
        if need <= n {
            res.update_max(n - need + packs * x);
        }
    }
    out_line!(res);
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
