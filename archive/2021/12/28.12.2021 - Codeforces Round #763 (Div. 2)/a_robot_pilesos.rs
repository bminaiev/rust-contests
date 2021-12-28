//{"name":"A. Робот-пылесос","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"http://codeforces.com/contest/1623/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10 10 6 1 2 8\n10 10 9 9 1 1\n9 8 5 6 2 1\n6 9 2 2 5 8\n2 2 1 1 2 1\n","output":"7\n10\n9\n3\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARobotPilesos"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use std::cmp::min;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let m = input.i32();
    let x1 = input.i32();
    let y1 = input.i32();
    let x2 = input.i32();
    let y2 = input.i32();
    let solve = |sz: i32, start: i32, end: i32| -> i32 {
        if start <= end {
            end - start
        } else {
            (sz - start) * 2 + (start - end)
        }
    };
    let res = min(solve(n, x1, x2), solve(m, y1, y2));
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
