//{"name":"A. Не красить","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n3 5 1 4\nWBWWW\nBBBWB\nWWBBB\n4 3 2 1\nBWW\nBBW\nWBB\nWWB\n2 3 2 2\nWWW\nWWW\n2 2 1 1\nWW\nWB\n5 9 5 9\nWWWWWWWWW\nWBWBWBBBW\nWBBBWWBWW\nWBWBWBBBW\nWWWWWWWWW\n1 1 1 1\nB\n1 1 1 1\nW\n1 2 1 1\nWB\n2 1 1 1\nW\nB\n","output":"1\n0\n-1\n2\n2\n0\n-1\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANeKrasit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

const BLACK: u8 = b'B';

fn solve_with_input(n: usize, m: usize, r: usize, c: usize, f: &[Vec<u8>]) -> i32 {
    if f[r][c] == BLACK {
        return 0;
    }
    if f[r].iter().any(|x| *x == BLACK) {
        return 1;
    }
    if (0..n).any(|pos| f[pos][c] == BLACK) {
        return 1;
    }
    for x in 0..n {
        for y in 0..m {
            if f[x][y] == BLACK {
                return 2;
            }
        }
    }
    return -1;
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let r = input.usize() - 1;
    let c = input.usize() - 1;
    let f = gen_vec(n, |_| input.string_as_vec());
    out_line!(solve_with_input(n, m, r, c, &f));
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
