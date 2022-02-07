//{"name":"A. Разворачивай и конкатенируй","group":"Codeforces - Codeforces Round #770 (Div. 2)","url":"https://codeforces.com/contest/1634/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 2\naab\n3 3\naab\n7 1\nabacaba\n2 0\nab\n","output":"2\n2\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARazvorachivaiIKonkatenirui"}}}

use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let s = input.string();
    if k == 0 || s.reversed() == s {
        out_line!(1);
    } else {
        out_line!(2);
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
