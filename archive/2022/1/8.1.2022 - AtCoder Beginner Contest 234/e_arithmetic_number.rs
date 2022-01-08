//{"name":"E - Arithmetic Number","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_e","interactive":false,"timeLimit":2000,"tests":[{"input":"152\n","output":"159\n"},{"input":"88\n","output":"88\n"},{"input":"8989898989\n","output":"9876543210\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EArithmeticNumber"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use algo_lib::misc::digits::{digits, char_from_digit};
use algo_lib::strings::utils::VecToString;

fn solve(input: &mut Input) {
    let number = input.i64();
    if number < 100 {
        out_line!(number);
        return;
    }
    let digits = digits(number);
    for len in digits.len().. {
        for first in 1..10 {
            for delta in -9..=9 {
                let last = first + delta * (len as i32 - 1);
                if last < 0 || last > 9 {
                    continue;
                }
                let check: Vec<_> = (0..len).map(|pos| first + (pos as i32) * delta).collect();
                if check.len() > digits.len() || check >= digits {
                    let as_str = check.into_iter().map(char_from_digit).collect::<Vec<_>>().to_string();
                    out_line!(as_str);
                    return;
                }
            }
        }
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
}
//END MAIN
