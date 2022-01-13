//{"name":"F. Интеракдивная задача","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/F","interactive":true,"timeLimit":1000,"tests":[{"input":"3\n\n1\n","output":"+ 1\n\n! 3\n"},{"input":"5\n\n0\n\n0\n\n1\n","output":"+ 1\n\n+ 1\n\n+ 1\n\n! 5\n"},{"input":"10\n\n0\n\n0\n\n1\n\n2\n","output":"+ 2\n\n+ 2\n\n+ 3\n\n+ 8\n\n! 20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FInterakdivnayaZadacha"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use std::ops::Range;

fn guess(range: Range<usize>, n: usize, input: &mut Input) {
    if range.len() == 1 {
        out_line!("!", range.start);
    } else {
        let next_n = (range.start + n) / n * n;
        let diff = next_n - range.start;
        let sub = range.len() / 2;
        let add = diff - sub;
        out_line!("+", add);
        output().flush();
        let next_range = range.start + add..range.end + add;
        if input.usize() * n == next_n {
            guess(next_n..next_range.end, n, input);
        } else {
            guess(next_range.start..next_n, n, input);
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    guess(1..n, n, input);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_locally();
}
//END MAIN
