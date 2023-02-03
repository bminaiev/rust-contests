//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn calc_substr(a: &[u32]) -> usize {
    let mut all = BTreeSet::new();
    for i in 0..a.len() {
        for j in i + 1..=a.len() {
            all.insert(a[i..j].to_owned());
        }
    }
    all.len()
}

fn solve_slow(n: usize) -> Vec<u32> {
    let mut res = vec![0; n];
    for mask in 0..1 << n {
        let mut a = vec![];
        for i in 0..n {
            if ((1 << i) & mask) != 0 {
                a.push(1);
            } else {
                a.push(0);
            }
        }
        if calc_substr(&a) > calc_substr(&res) {
            res = a;
        }
    }
    res
}

fn stress() {
    for n in 1..20 {
        let res = solve_slow(n);
        dbg!(n, res, calc_substr(&res), n * (n - 1) / 2);
    }
}

fn solve(input: &mut Input, _test_case: usize) {}

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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
