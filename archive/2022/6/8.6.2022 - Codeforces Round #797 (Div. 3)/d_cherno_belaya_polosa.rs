//{"name":"D. Черно-белая полоса","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 3\nBBWBW\n5 5\nBBWBW\n5 1\nBBWBW\n1 1\nW\n","output":"1\n2\n0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DChernoBelayaPolosa"}}}

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
    let k = input.usize();
    let s: Vec<_> = input
        .string()
        .into_iter()
        .map(|c| if c == b'B' { 1 } else { 0 })
        .collect();
    let pref = s.pref_sum();
    let mut best = std::usize::MAX;
    for last in k..=n {
        let cur = pref[last] - pref[last - k];
        best.update_min(k - cur);
    }
    out_line!(best);
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
