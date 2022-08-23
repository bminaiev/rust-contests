//{"name":"Subtract 12 Operation","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems/SUB12OP","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n2 4\n3\n1 1 1\n6\n-4 2 -4 2 -4 2\n1\n-100000000\n","output":"0\n2\n15\n100000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Subtract12Operation"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut dp = HashMap::new();
    dp.insert(0, 0);
    for pos in (1..n).rev() {
        let mut ndp = HashMap::new();
        let mut update_min = |k: i64, v: i64| {
            let entry = ndp.entry(k).or_insert(std::i64::MAX / 2);
            if *entry > v {
                *entry = v;
            }
        };
        for (k, v) in dp.into_iter() {
            let cur_val = a[pos] - k;
            if cur_val > 0 {
                let full = cur_val / 2;
                update_min(full, (cur_val - full * 2).abs() + v);
                if cur_val % 2 != 0 {
                    update_min(full + 1, (cur_val - full * 2 - 2).abs() + v);
                }
            } else {
                update_min(0, cur_val.abs() + v);
            }
        }
        dp = ndp;
    }
    let mut res = std::i64::MAX;
    for (k, v) in dp.into_iter() {
        res.update_min((a[0] - k).abs() + v);
    }
    out_line!(res);
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
}
//END MAIN
