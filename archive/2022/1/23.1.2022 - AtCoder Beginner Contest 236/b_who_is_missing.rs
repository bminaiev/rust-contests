//{"name":"B - Who is missing?","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3 2 3 3 2 2 1 1 1 2\n","output":"3\n"},{"input":"1\n1 1 1\n","output":"1\n"},{"input":"4\n3 2 1 1 2 4 4 4 4 3 1 3 2 1 3\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BWhoIsMissing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize() * 4 - 1;
    let mut res = 0;
    for _ in 0..n {
        res ^= input.i64();
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
