//{"name":"B. Susu Legend","group":"TLX - TLX Regular Open Contest #30","url":"https://tlx.toki.id/contests/troc-30/problems/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4 1 4 1 2 2\n1 1 1 4 4 2\n","output":"YES\n"},{"input":"3\n2 3 1\n3 1 2\n","output":"NO\n"},{"input":"4\n3 3 4 3\n3 3 2 3\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSusuLegend"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    let b = input.vec::<i32>(n);
    let mut it = 0;
    for i in 0..n {
        while it != n && a[it] != b[i] {
            it += 1;
        }
        if it == n {
            out_line!("NO");
            return;
        }
    }
    out_line!("YES");
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
