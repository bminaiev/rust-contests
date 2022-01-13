//{"name":"B. И не ноль","group":"Codeforces - Codeforces Global Round 18","url":"https://codeforces.com/contest/1615/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2\n2 8\n4 5\n1 5\n100000 200000\n","output":"1\n3\n0\n2\n31072\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BINeNol"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use std::cmp::max;

fn solve(input: &mut Input, _test_case: usize) {
    let l = input.i32();
    let r = input.i32();
    let count = |till: i32, bit: usize| {
        let pw = 1 << bit;
        till / (pw * 2) * pw + max(0, till % (pw * 2) - pw)
    };
    let max_stay = (0..30)
        .map(|bit| count(r + 1, bit) - count(l, bit))
        .max()
        .unwrap();
    out_line!(r - l + 1 - max_stay);
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
