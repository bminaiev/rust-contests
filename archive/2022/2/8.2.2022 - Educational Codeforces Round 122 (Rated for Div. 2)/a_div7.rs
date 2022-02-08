//{"name":"A. Div. 7","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n42\n23\n377\n","output":"42\n28\n777\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADiv7"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let x = input.i64();
    if x % 7 == 0 {
        out_line!(x);
    } else {
        for last in 0..10 {
            let check = (x / 10 * 10) + last;
            if check % 7 == 0 {
                out_line!(check);
                return;
            }
        }
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
