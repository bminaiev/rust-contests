//{"name":"F. Съедая конфеты","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n10 20 10\n6\n2 1 4 2 4 1\n5\n1 2 4 8 16\n9\n7 3 20 5 15 1 11 8 10\n","output":"2\n6\n0\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSedayaKonfeti"}}}

use std::collections::{HashMap, HashSet};

use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::pref_sum::PrefSum;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let pref = a.pref_sum();
    let mut seen = HashMap::new();
    for (cnt, &sum) in a.reversed().pref_sum().iter().enumerate() {
        seen.insert(sum, cnt);
    }
    let mut res = 0;
    for (cnt, sum) in pref.iter().enumerate() {
        if let Some(cnt2) = seen.get(sum) {
            if cnt + cnt2 <= n {
                res.update_max(cnt + cnt2);
            }
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
}
//END MAIN
