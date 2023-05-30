//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let thousands = ["", "M", "MM", "MMM"];
    let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let mut hm = HashMap::<_, usize>::new();
    for value in 1..4000 {
        let mut s = String::new();
        s.push_str(thousands[value / 1000]);
        s.push_str(hundreds[(value % 1000) / 100]);
        s.push_str(tens[(value % 100) / 10]);
        s.push_str(ones[value % 10]);
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.sort();
        *hm.entry(chars).or_default() += 1;
    }
    let tc = input.usize();
    for _ in 0..tc {
        let s = input.string_as_string();
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.sort();
        let value = *hm.get(&chars).unwrap_or(&0);
        out_line!(value);
    }
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
