//{"name":"B - Matrix Transposition","group":"AtCoder - AtCoder Beginner Contest 237","url":"https://atcoder.jp/contests/abc237/tasks/abc237_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n1 2 3\n4 5 6\n7 8 9\n10 11 12\n","output":"1 4 7 10\n2 5 8 11\n3 6 9 12\n"},{"input":"2 2\n1000000000 1000000000\n1000000000 1000000000\n","output":"1000000000 1000000000\n1000000000 1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMatrixTransposition"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let arr = input.read_matrix::<i64>(n, m);
    let res = arr.transpose();
    out_line!(res);
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
