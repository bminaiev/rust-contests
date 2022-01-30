//{"name":"C. Странная контрольная","group":"Codeforces - Codeforces Round #769 (Div. 2)","url":"https://codeforces.com/contest/1632/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 3\n5 8\n2 5\n3 19\n56678 164422\n","output":"1\n3\n2\n1\n23329\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CStrannayaKontrolnaya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.i32();
    let b = input.i32();
    let mut res = b - a;
    for a2 in a..b {
        if a2 | b == b {
            res.update_min(a2 - a + 1);
        } else {
            let mut max_changed_bit = 25;
            while ((1 << max_changed_bit) & a2) == 0 || ((1 << max_changed_bit) & b) != 0 {
                max_changed_bit -= 1;
            }
            let mask = (1 << max_changed_bit) * 2 - 1;
            let need_b = (b & !mask) | (a2 & mask);
            res.update_min(a2 - a + need_b - b + 1);
        }
    }
    out_line!(res);
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
