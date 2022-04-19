//{"name":"Weightlifting","group":"Google Coding Competitions - Round 1A 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000877ba5/0000000000aa9280","interactive":false,"timeLimit":20000,"tests":[{"input":"3\n3 1\n1\n2\n1\n2 3\n1 2 1\n2 1 2\n3 3\n3 1 1\n3 3 3\n2 3 3\n","output":"Case #1: 4\nCase #2: 12\nCase #3: 20\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Weightlifting"}}}

use std::cmp::min;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let days = input.usize();
    let n = input.usize();
    let a = input.matrix::<i64>(days, n);
    let mut same_cnt = Array2D::new(0, days, days);
    for first in 0..days {
        let mut smallest: Vec<_> = a[first].iter().cloned().collect();
        let mut cur_sum = smallest.iter().sum::<i64>();
        for second in first..days {
            for i in 0..n {
                let nsmall = min(smallest[i], a[second][i]);
                cur_sum -= smallest[i] - nsmall;
                smallest[i] = nsmall;
            }
            same_cnt[first][second] = cur_sum;
        }
    }
    let mut dp = Array2D::new(std::i64::MAX / 3, days, days);
    for i in 0..days {
        dp[i][i] = same_cnt[i][i];
    }
    for len in 2..=days {
        for first in 0..=(days - len) {
            let last = first + len - 1;
            for mid in first..last {
                let cur = dp[first][mid] + dp[mid + 1][last] - same_cnt[first][last];
                dp[first][last].update_min(cur);
            }
        }
    }
    out_line!(format!("Case #{}: {}", test_case, dp[0][days - 1] * 2));
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
