//{"name":"A. Равноудаленные буквы","group":"Codeforces - Educational Codeforces Round 121 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1626/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"3\noelhl\nabcdcba\nac\n","output":"hello\nababcdc\nac\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARavnoudalennieBukvi"}}}

use algo_lib::collections::value_to_positions::calc_value_occurrences;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string_as_vec();
    let mut cnt = calc_value_occurrences(&s);
    let mut res = vec![];
    for x in 0..cnt.len() {
        if cnt[x as usize] == 1 {
            res.push(x as u8);
            cnt[x as usize] -= 1;
        }
    }
    for _ in 0..2 {
        for x in 0..cnt.len() {
            if cnt[x as usize] > 0 {
                res.push(x as u8);
                cnt[x as usize] -= 1;
            }
        }
    }
    out_line!(vec2str(&res));
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
