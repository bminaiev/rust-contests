//{"name":"B. Гадание на массиве","group":"Codeforces - Codeforces Round #770 (Div. 2)","url":"https://codeforces.com/contest/1634/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 7 9\n2\n2 0 2\n1 3\n4 0 1\n1 2 3 4\n2 1000000000 3000000000\n1000000000 1000000000\n","output":"Alice\nAlice\nBob\nAlice\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGadanieNaMassive"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut x = input.i64() % 2;
    let y = input.i64() % 2;
    let a: Vec<i64> = input.read_vec(n);
    for &val in a.iter() {
        x = (x + val) % 2;
    }
    if x == y {
        out_line!("Alice");
    } else {
        out_line!("Bob");
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
