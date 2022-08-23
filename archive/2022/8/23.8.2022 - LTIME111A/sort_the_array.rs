//{"name":"Sort the Array","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems/SORTARRAY","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6\n1 2 1 3 4 3\n3\n3 1 2\n","output":"2\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SortTheArray"}}}

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
    let a = input.vec::<usize>(n);
    const MAX: usize = std::usize::MAX / 2;
    let mut dp: Vec<usize> = vec![MAX; a.len() + 1];
    dp[n] = 0;
    dp[n - 1] = 0;
    let mut seen = HashMap::new();
    for pos in (0..n).rev() {
        if pos + 1 != n && a[pos] <= a[pos + 1] {
            dp[pos] = dp[pos + 1];
        }
        if let Some(&prev_pos) = seen.get(&a[pos]) {
            let cur = 1 + dp[prev_pos];
            dp[pos].update_min(cur);
            let cur: usize = dp[pos];
            if cur < dp[prev_pos] {
                dp[prev_pos] = cur;
            }
        } else {
            seen.insert(a[pos], pos);
        }
    }
    if dp[0] >= MAX {
        out_line!(-1);
    } else {
        out_line!(dp[0]);
    }
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
