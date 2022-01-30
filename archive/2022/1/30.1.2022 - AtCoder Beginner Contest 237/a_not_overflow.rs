//{"name":"A - Not Overflow","group":"AtCoder - AtCoder Beginner Contest 237","url":"https://atcoder.jp/contests/abc237/tasks/abc237_a","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n","output":"Yes\n"},{"input":"-9876543210\n","output":"No\n"},{"input":"483597848400000\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANotOverflow"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.i64();
    const MAX: i64 = (1 << 31);
    if n >= -MAX && n <= MAX - 1 {
        out_line!("Yes");
    } else {
        out_line!("No");
    }
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
    // tester::run_single_test("1");
}
//END MAIN
