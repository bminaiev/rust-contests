//{"name":"nerc_delete","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"nerc_delete"}}}

use std::collections::HashMap;

use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let t = input.string();
    let mut count: HashMap<u8, i32> = HashMap::new();
    for &c in t.iter() {
        *count.entry(c).or_default() += 1;
    }
    let s_filtered: Vec<_> = s
        .iter()
        .cloned()
        .rev()
        .filter(|&c| -> bool {
            let entry = count.entry(c).or_default();
            *entry -= 1;
            *entry >= 0
        })
        .collect();
    if s_filtered == t.reversed() {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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
