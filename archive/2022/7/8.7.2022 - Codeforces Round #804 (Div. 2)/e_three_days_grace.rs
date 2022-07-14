//{"name":"E. Three Days Grace","group":"Codeforces - Codeforces Round #804 (Div. 2)","url":"https://codeforces.com/contest/1699/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n5 10\n2 4 2 4 2\n3 50\n12 2 3\n2 40\n6 35\n2 5\n1 5\n","output":"0\n1\n2\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EThreeDaysGrace"}}}

use std::cmp::min;

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize() + 1;
    let mut exist_value = BitSet::new(m);
    for _ in 0..n {
        exist_value.set(input.usize(), true);
    }
    let mut balance = vec![0; m];
    balance[0] = (0..m).map(|p| if exist_value.get(p) { 1 } else { 0 }).sum();
    let mut min_it = 0;
    let mut dp = vec![0; m];
    let mut res = std::usize::MAX;
    for max_div in 1..m {
        for val in (max_div..m).step_by(max_div) {
            let more = val / max_div;
            let mut next_val = min(dp[more], max_div);
            if val == max_div {
                next_val = max_div;
            }
            let need_change = exist_value.get(val);
            if next_val > dp[val] {
                if need_change {
                    balance[dp[val]] -= 1;
                }
                dp[val] = next_val;
                if need_change {
                    balance[dp[val]] += 1;
                }
            }
        }
        while balance[min_it] == 0 {
            min_it += 1;
        }
        if min_it != 0 {
            res.update_min(max_div - min_it);
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

// fn stress() {
//     const MAX: usize = 5_000_005;
//     let mut g = vec![vec![]; MAX];
//     for x in 2..MAX {
//         for y in (x * x..MAX).step_by(x) {
//             g[y].push(x);
//         }
//     }
//     let mut sum_sz = 0;
//     for gg in g.iter() {
//         sum_sz += gg.len();
//     }
//     dbg!(sum_sz);
// }

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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
