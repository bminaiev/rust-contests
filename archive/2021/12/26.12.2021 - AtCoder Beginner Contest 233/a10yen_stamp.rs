//{"name":"A - 10yen Stamp","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_a","interactive":false,"timeLimit":2000,"tests":[{"input":"80 94\n","output":"2\n"},{"input":"1000 63\n","output":"0\n"},{"input":"270 750\n","output":"48\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A10yenStamp"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let x = input.i64();
    let y = input.i64();
    let res = if x >= y { 0 } else { (y - x + 9) / 10 };
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
