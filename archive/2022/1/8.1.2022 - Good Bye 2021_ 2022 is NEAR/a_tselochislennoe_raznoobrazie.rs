//{"name":"A. Целочисленное разнообразие","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"http://codeforces.com/contest/1616/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 1 2 2\n3\n1 2 3\n2\n0 0\n","output":"4\n3\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATselochislennoeRaznoobrazie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut table = HashSet::new();
    for _ in 0..n {
        let x = input.i32();
        if table.contains(&x) {
            table.insert(-x);
        } else {
            table.insert(x);
        }
    }
    out_line!(table.len());
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
