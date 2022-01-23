//{"name":"C - Route Map","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\ntokyo kanda akiba okachi ueno\ntokyo akiba ueno\n","output":"Yes\nNo\nYes\nNo\nYes\n"},{"input":"7 7\na t c o d e r\na t c o d e r\n","output":"Yes\nYes\nYes\nYes\nYes\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRouteMap"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};
use std::collections::BTreeSet;

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let s = gen_vec(n, |_| input.string_as_vec());
    let t = gen_vec(m, |_| input.string_as_vec());
    let mut seen = BTreeSet::new();
    for x in t.into_iter() {
        seen.insert(x);
    }
    for check in s.iter() {
        if seen.contains(check) {
            out_line!("Yes");
        } else {
            out_line!("No");
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
