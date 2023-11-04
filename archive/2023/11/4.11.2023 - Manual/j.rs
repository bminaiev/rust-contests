//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::intervals_set::IntervalsSet;
use algo_lib::seg_trees::lazy_fenwick::LazyFenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let q = input.usize();

    let mut intervals = HashMap::<i32, IntervalsSet<i32>>::new();
    let mut fenwick = LazyFenwick::new(n);

    for _ in 0..q {
        let qtype = input.string_as_string();
        match qtype.as_str() {
            "+" => {
                let l = input.i32() - 1;
                let r = input.i32();
                let x = input.i32();

                intervals.entry(x).or_default().insert(l..r, |range| {
                    fenwick.add(range.start, 1);
                    fenwick.add(range.end, -1);
                });
            }
            "-" => {
                let l = input.i32() - 1;
                let r = input.i32();
                let x = input.i32();

                intervals.entry(x).or_default().remove(l..r, |range| {
                    fenwick.add(range.start, -1);
                    fenwick.add(range.end, 1);
                });
            }
            "?" => {
                let pos = input.i32() - 1;
                let res = fenwick.get_sum(pos);
                out_line!(res);
                output().flush();
            }
            _ => unreachable!(),
        }
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
