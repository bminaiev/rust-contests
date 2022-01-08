//{"name":"C - Happy New Year!","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n","output":"22\n"},{"input":"11\n","output":"2022\n"},{"input":"923423423420220108\n","output":"220022020000202020002022022000002020002222002200002022002200\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CHappyNewYear"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::digits::digits_base;
use algo_lib::strings::utils::VecToString;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let k = input.i64();
    let as_str = digits_base(k, 2)
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<_>>()
        .to_string();
    out_line!(as_str);
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
