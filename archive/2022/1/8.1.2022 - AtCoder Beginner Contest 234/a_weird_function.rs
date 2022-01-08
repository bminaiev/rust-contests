//{"name":"A - Weird Function","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_a","interactive":false,"timeLimit":2000,"tests":[{"input":"0\n","output":"1371\n"},{"input":"3\n","output":"722502\n"},{"input":"10\n","output":"1111355571\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AWeirdFunction"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn f(x: i64) -> i64 {
    x * x + 2 * x + 3
}

fn solve(input: &mut Input) {
    let t = input.i64();
    let res = f(f(f(t) + t) + f(f(t)));
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
