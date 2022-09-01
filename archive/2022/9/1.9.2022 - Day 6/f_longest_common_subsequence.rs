//{"name":"F. Longest Common Subsequence","group":"Yandex - Day 6","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39551/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 3 1024 1 1 1 1\n3 4 1024 0 0 0 0\n","output":"0\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLongestCommonSubsequence"}}}

use std::cmp::min;
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
    let m = input.usize();
    let p = input.i64();
    let mut x = input.i64();
    let a = input.i64();
    let b = input.i64();
    let c = input.i64();

    let mut gen = |n: usize| -> Vec<i64> {
        let mut res = vec![];
        for _ in 0..n {
            let x2 = x * x % p;
            x = (a * x2 + b * x + c) % p;
            res.push(x);
        }
        res
    };
    let a = gen(n);
    let b = gen(m);
    let mut pos = HashMap::new();
    for i in (0..a.len()).rev() {
        pos.insert(a[i], i);
    }
    let mut res = 0;
    for i in 0..b.len() {
        if let Some(&p_a) = pos.get(&b[i]) {
            let max_len = min(n - p_a, m - i);
            res.update_max(max_len);
        }
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
