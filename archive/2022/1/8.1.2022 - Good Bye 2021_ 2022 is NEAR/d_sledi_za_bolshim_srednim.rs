//{"name":"D. Следи за большим средним","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"http://codeforces.com/contest/1616/problem/D","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n5\n1 2 3 4 5\n2\n10\n2 4 2 4 2 4 2 4 2 4\n3\n3\n-10 -5 -10\n-8\n3\n9 9 -3\n5\n","output":"4\n8\n2\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSlediZaBolshimSrednim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};
use std::cmp::{max, min};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i64>(n);
    let x = input.i64();
    let mut dp = vec![0; 3];
    for pos in 0..n {
        let mut ndp = vec![0; 3];
        for prev in 0..3 {
            if prev > pos {
                continue;
            }
            for use_new in 0..=1 {
                let next = if use_new == 1 { min(2, prev + 1) } else { 0 };
                if use_new == 1 && prev >= 1 && a[pos] + a[pos - 1] < 2 * x {
                    continue;
                }
                if use_new == 1 && prev >= 2 && a[pos] + a[pos - 1] + a[pos - 2] < 3 * x {
                    continue;
                }
                ndp[next].update_max(dp[prev] + use_new);
            }
        }
        dp = ndp;
    }
    let res = max(dp[0], max(dp[1], dp[2]));
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
