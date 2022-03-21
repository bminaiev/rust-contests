//{"name":"E. Арифметические операции","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"http://codeforces.com/contest/1654/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"9\n3 2 7 8 6 9 5 4 1\n","output":"6\n"},{"input":"14\n19 2 15 8 9 14 17 13 4 14 4 11 15 7\n","output":"10\n"},{"input":"10\n100000 1 60000 2 20000 4 8 16 32 64\n","output":"7\n"},{"input":"4\n10000 20000 10000 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EArifmeticheskieOperatsii"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_positive_delta(a: &[i32]) -> usize {
    let mut res = a.len();
    const BUBEN: i32 = 200;
    for delta in 0..BUBEN {
        let mut seen_start: FxHashMap<i32, usize> = FxHashMap::default();
        for (pos, &val) in a.iter().enumerate() {
            let start = val - (pos as i32) * delta;
            *seen_start.entry(start).or_default() += 1;
        }
        for cnt in seen_start.values() {
            res.update_min(a.len() - *cnt);
        }
    }
    const MAX: i32 = 1.1e5 as i32;
    for pos in 0..a.len() {
        let mut seen_delta: FxHashMap<i32, usize> = FxHashMap::default();
        for pos2 in pos + 1..a.len() {
            let delta_pos = (pos2 - pos) as i32;
            if delta_pos * BUBEN > MAX {
                break;
            }
            if (a[pos2] - a[pos]) % delta_pos == 0 {
                let delta = (a[pos2] - a[pos]) / delta_pos;
                *seen_delta.entry(delta).or_default() += 1;
            }
        }
        for cnt in seen_delta.values() {
            res.update_min(a.len() - *cnt - 1);
        }
    }
    res
}

fn stress() {
    let n = 100_000;
    let a: Vec<_> = (0..n as i32).collect();
    solve_positive_delta(&a);
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    let mut res = solve_positive_delta(&a);
    a.reverse();
    res.update_min(solve_positive_delta(&a));
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
