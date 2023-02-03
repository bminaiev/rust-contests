//{"name":"A. Ноутбук и проектор","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, основан на Отборочном раунде VK Cup 2022)","url":"https://codeforces.com/contest/1782/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n55 20 29\n23 10 18 3\n20 10 5\n1 5 2 5\n15 15 4\n7 13 10 10\n2 1000 2\n1 1 1 999\n10 4 10\n7 1 2 1\n","output":"47\n8\n14\n1002\n17\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANoutbukIProektor"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn calc(a: i32, b: i32, f: i32, g: i32) -> i32 {
    let s1 = a + f + (b - g).abs();
    let s2 = b + g + (a - f).abs();
    min(s1, s2)
}

fn solve(input: &mut Input, _test_case: usize) {
    let w = input.i32();
    let d = input.i32();
    let h = input.i32();
    let a = input.i32();
    let b = input.i32();
    let f = input.i32();
    let g = input.i32();
    let mut res = i32::MAX;
    res.update_min(calc(a, b, f, g));
    res.update_min(calc(w - a, b, w - f, g));
    res.update_min(calc(a, d - b, f, d - g));
    res.update_min(calc(w - a, d - b, w - f, d - g));
    res += h;
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
