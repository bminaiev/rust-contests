//{"name":"A. Сокращение разрыва","group":"Codeforces - Codeforces Global Round 18","url":"https://codeforces.com/contest/1615/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n10 10 10\n4\n3 2 1 2\n5\n1 2 3 1 5\n","output":"0\n0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASokrashchenieRazriva"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let sum: i64 = input.read_vec::<i64>(n).iter().sum();
    if sum % (n as i64) == 0 {
        out_line!(0);
    } else {
        out_line!(1);
    }
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
