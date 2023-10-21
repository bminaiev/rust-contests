//{"name":"A. Шлемы в светлой ночи","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 3\n2 3 2 1 1 3\n4 3 2 6 3 6\n1 100000\n100000\n1\n4 94\n1 4 2 3\n103 96 86 57\n","output":"16\n100000\n265\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AShlemiVSvetloiNochi"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    cost: i64,
    cnt: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p = input.i64();
    let cnt = input.vec::<i64>(n);
    let cost = input.vec::<i64>(n);
    let mut a = gen_vec(n, |i| Person {
        cost: cost[i],
        cnt: cnt[i],
    });
    a.sort();
    let mut ok = 0;
    let mut cost = 0;
    for i in 0..n {
        if ok <= i as i64 {
            ok += 1;
            cost += p;
        }
        let use_here = min((n as i64) - ok, a[i].cnt);
        ok += use_here;
        cost += use_here * min(p, a[i].cost);
    }
    out_line!(cost);
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
