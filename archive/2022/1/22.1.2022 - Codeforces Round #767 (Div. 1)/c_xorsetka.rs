//{"name":"C. XOR-сетка","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 5\n5 1\n4\n1 14 8 9\n3 1 5 9\n4 13 11 1\n1 15 4 11\n4\n2 4 1 6\n3 7 3 10\n15 9 4 2\n12 7 15 1\n","output":"4\n9\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CXORSetka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::step_down::step_down;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |_| input.read_vec::<i32>(n));
    let mut res = 0;
    for _ in 0..2 {
        let mut add = |r: usize, c: usize| {
            res ^= a[r][c];
        };
        let mut shift = 0;
        for len in step_down(n, 2, 4) {
            add(shift, shift);
            for delta in (0..len).step_by(4).skip(1) {
                add(shift + delta, shift);
                add(shift, shift + delta);
            }
            if len != 2 {
                let last = shift + len - 1;
                for delta in (2..=len - 2).step_by(4) {
                    add(last - delta, last);
                    add(last, last - delta);
                }
            }

            shift += 2;
        }
        a.reverse();
    }
    out_line!(res);
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
