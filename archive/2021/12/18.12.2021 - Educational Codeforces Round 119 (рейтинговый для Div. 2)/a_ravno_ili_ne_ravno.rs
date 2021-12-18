//{"name":"A. Равно или не равно","group":"Codeforces - Educational Codeforces Round 119 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1620/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nEEE\nEN\nENNEENE\nNENN\n","output":"YES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARavnoIliNeRavno"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string_as_vec();
    let cnt_no = s.iter().filter(|&c| c == &b'N').count();
    if cnt_no == 1 {
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
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
