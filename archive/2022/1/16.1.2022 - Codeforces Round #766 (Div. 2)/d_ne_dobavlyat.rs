//{"name":"D. Не добавлять","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 20 1 25 30\n","output":"3\n"},{"input":"3\n6 10 15\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNeDobavlyat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::gcd::gcd;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a: Vec<usize> = input.read_vec(n);
    let max = a.iter().max().unwrap() + 1;
    let mut seen = vec![false; max];
    for &x in a.iter() {
        seen[x] = true;
    }
    let mut cnt = 0;
    for val in (1..max).rev() {
        let mut g = 0;
        for next in (val..max).step_by(val) {
            if seen[next] {
                g = gcd(g, next);
            }
        }
        if g == val {
            seen[val] = true;
            cnt += 1;
        }
    }
    out_line!(cnt - a.len());
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
    tester::run_tests();
}
//END MAIN
