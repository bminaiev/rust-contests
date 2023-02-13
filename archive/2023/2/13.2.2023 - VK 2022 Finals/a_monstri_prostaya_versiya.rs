//{"name":"A. Монстры (простая версия)","group":"Codeforces - VK 2022 Finals","url":"https://codeforces.com/gym/425375/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4\n3 1 7 1\n6\n5 5 1 5 5 5\n","output":"5\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMonstriProstayaVersiya"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i64>(n);
    a.sort();
    let mut prev = 0;
    let mut res = 0;
    for &x in a.iter() {
        if x <= prev {
            continue;
        }
        prev = prev + 1;
        res += x - prev;
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
