//{"name":"G - Good Vertices","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_g","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5 3\n2 3\n3 4\n1 2\n3 2\n2 2\n","output":"-1 4 5 3\n"},{"input":"2 1 1000000000\n1 2\n","output":"-1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGoodVertices"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};
use std::cmp::max;

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let len = input.i64();

    const MAX: usize = std::usize::MAX;

    let mut g = Array2D::new(MAX, n, n);
    for time in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr][to] = time;
    }
    let mult = |a: &Array2D<usize>, b: &Array2D<usize>| -> Array2D<usize> {
        let n = a.len();
        let m = a[0].len();
        let k2 = b[0].len();
        let mut res = Array2D::new(MAX, n, k2);
        for i in 0..n {
            for j in 0..m {
                for k in 0..k2 {
                    res[i][k].update_min(max(a[i][j], b[j][k]));
                }
            }
        }
        res
    };
    let mut init = Array2D::new(MAX, 1, n);
    init[0][0] = 0;
    for pw in 0..31 {
        if (1 << pw) & len != 0 {
            init = mult(&init, &g);
        }
        g = mult(&g, &g);
    }
    for &x in init[0].iter() {
        if x == MAX {
            out!(-1, "");
        } else {
            out!(x + 1, "");
        }
    }
    out_line!();
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
