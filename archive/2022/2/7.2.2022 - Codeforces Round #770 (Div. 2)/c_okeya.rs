//{"name":"C. ОКЕЯ","group":"Codeforces - Codeforces Round #770 (Div. 2)","url":"https://codeforces.com/contest/1634/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n2 2\n3 3\n3 1\n","output":"YES\n1\nYES\n1 3\n2 4\nNO\nYES\n1\n2\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COKEYa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, dbg};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    if k != 1 && n % 2 != 0 {
        out_line!("NO");
    } else {
        out_line!("YES");
        if k == 1 {
            for i in 1..=n {
                out_line!(i);
            }
        } else {
            for row in (0..n).step_by(2) {
                let from = row * k;
                for i in 0..k {
                    out!(from + i * 2 + 1, "");
                }
                out_line!();
                for i in 0..k {
                    out!(from + i * 2 + 2, "");
                }
                out_line!();
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
