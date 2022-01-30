//{"name":"C - kasaka","group":"AtCoder - AtCoder Beginner Contest 237","url":"https://atcoder.jp/contests/abc237/tasks/abc237_c","interactive":false,"timeLimit":2000,"tests":[{"input":"kasaka\n","output":"Yes\n"},{"input":"atcoder\n","output":"No\n"},{"input":"php\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKasaka"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use std::collections::VecDeque;

fn solve(input: &mut Input) {
    let s = input.string_as_vec();
    let mut deque: VecDeque<u8> = s.into_iter().collect();
    while deque.back() == Some(&b'a') {
        deque.pop_back();
        if deque.front() == Some(&b'a') {
            deque.pop_front();
        }
    }
    let vec: Vec<_> = deque.into_iter().collect();
    if vec.reversed() == vec {
        out_line!("Yes");
    } else {
        out_line!("No");
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
    // tester::run_single_test("1");
}
//END MAIN
