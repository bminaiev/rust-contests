//{"name":"D1. Игра на сумму (Легкая версия)","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/D1","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3 3 2\n2 1 10\n6 3 10\n6 4 10\n100 1 1\n4 4 0\n69 4 20\n","output":"6\n5\n375000012\n500000026\n958557139\n0\n49735962\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1IgraNaSummuLegkayaVersiya"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod7;
use algo_lib::{dbg, out, out_line};
use std::cmp::min;

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k: Mod = input.read();
    let mut dp = Array2D::new(Mod::ZERO, n + 1, m + 1);
    for x in 1..=n {
        for y in 1..=min(x, m) {
            if x == y {
                dp[x][y] = Mod::new(x as i32);
            } else {
                dp[x][y] = (dp[x - 1][y - 1] + dp[x - 1][y]) / Mod::TWO;
            }
        }
    }
    let res = k * dp[n][m];
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
