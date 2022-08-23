//{"name":"A1. Бурёнка и традиции (простая версия)","group":"Codeforces - Codeforces Round #814 (Div. 1)","url":"https://codeforces.com/contest/1718/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n4\n5 5 5 5\n3\n1 3 2\n2\n0 0\n3\n2 5 7\n6\n1 2 3 3 2 1\n10\n27 27 34 32 2 31 23 56 52 4\n5\n1822 1799 57 23 55\n","output":"2\n2\n0\n2\n4\n7\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A1BuryonkaITraditsiiProstayaVersiya"}}}

use std::collections::{BTreeMap, BTreeSet};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    let mut res = 0;
    let mut seen = BTreeMap::new();
    let mut cur_xor = 0;
    let mut dp = vec![0; a.len() + 1];
    seen.insert(0, 0);
    for pos in 0..n {
        cur_xor ^= a[pos];
        dp[pos + 1] = dp[pos] + 1;
        if let Some(prev_pos) = seen.get(&cur_xor) {
            let len = pos + 1 - *prev_pos;
            let prev_dp = dp[*prev_pos];
            dp[pos + 1].update_min(prev_dp + len - 1);
        }
        seen.insert(cur_xor, pos + 1);
    }
    out_line!(dp[n]);
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
    // tester::run_stress(stress);
}
//END MAIN
