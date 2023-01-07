//{"name":"D. Superpainter","group":"TLX - TLX Regular Open Contest #30","url":"https://tlx.toki.id/contests/troc-30/problems/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6 3\n-1 3 -1 -1 6 0\n","output":"9\n"},{"input":"5 2\n-1 -1 -1 -1 -1\n","output":"4\n"},{"input":"15 10\n-1 980 -1 -1 -1 11 -1 1004 569 -1 -1 725 963 -1 -1\n","output":"335544304\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSuperpainter"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<i32>(n);
    let mut res = Mod::ONE;
    for i in 0..a.len() - 1 {
        if a[i] == -1 {
            continue;
        }
        if a[i + 1] != -1 {
            continue;
        }
        let mut j = i + 1;
        while j != a.len() && a[j] == -1 {
            j += 1;
        }
        if j != a.len() {
            let len = j - i;
            for bit in 0..m {
                if ((1 << bit) & a[i]) != ((1 << bit) & a[j]) {
                    res *= Mod::new(len);
                }
            }
        }
    }
    if a.iter().all(|v| *v == -1) {
        res = Mod::new(1 << m);
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
