//{"name":"A - chukodai","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_a","interactive":false,"timeLimit":2000,"tests":[{"input":"chokudai\n3 5\n","output":"chukodai\n"},{"input":"aa\n1 2\n","output":"aa\n"},{"input":"aaaabbbb\n1 8\n","output":"baaabbba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AChukodai"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let mut s = input.string_as_vec();
    let (p1, p2) = input.read();
    s.swap(p1 - 1, p2 - 1);
    out_line!(vec2str(&s));
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
