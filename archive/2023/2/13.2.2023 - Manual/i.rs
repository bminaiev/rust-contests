//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use std::cmp::max;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Person {
    add: i64,
    h: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let cnt = input.usize();
    let mut a = gen_vec(n, |_| Person {
        add: input.read(),
        h: input.read(),
    });
    a.sort_by_key(|p| p.h);
    const MX: i64 = std::i64::MIN / 100;
    let mut dp = vec![MX; n];
    for i in 0..n {
        dp[i] = a[i].add + a[i].h;
    }
    for _it in 0..cnt - 1 {
        let mut prev_best = MX;
        for j in 0..n {
            let new_prev_best = max(prev_best, dp[j]);
            dp[j] = prev_best + a[j].add;
            prev_best = new_prev_best;
        }
    }
    for i in 0..n {
        dp[i] -= a[i].h;
    }
    let res = *dp.iter().max().unwrap();
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
