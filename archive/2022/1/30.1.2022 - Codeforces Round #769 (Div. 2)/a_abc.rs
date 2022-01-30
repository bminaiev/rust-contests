//{"name":"A. ABC","group":"Codeforces - Codeforces Round #769 (Div. 2)","url":"https://codeforces.com/contest/1632/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n1\n2\n10\n2\n01\n4\n1010\n","output":"YES\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AABC"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    input.usize();
    let s = input.string_as_vec();
    if s.len() >= 3 || (s.len() == 2 && s[0] == s[1]) {
        out_line!("NO");
    } else {
        out_line!("YES");
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
    // tester::run_single_test("1");
}
//END MAIN
