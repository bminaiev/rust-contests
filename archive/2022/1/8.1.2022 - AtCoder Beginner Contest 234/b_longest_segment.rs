//{"name":"B - Longest Segment","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n0 0\n0 1\n1 1\n","output":"1.4142135624\n"},{"input":"5\n315 271\n-2 -621\n-205 -511\n-952 482\n165 463\n","output":"1455.7159750446\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLongestSegment"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

fn solve(input: &mut Input) {
    let n = input.usize();
    let a : Vec<Point> = input.read_vec(n);
    let mut res = 0;
    for p1 in a.iter() {
        for p2 in a.iter() {
            res.update_max(p1.dist2(p2));
        }
    }
    let ans = (res as f64).sqrt();
    out_line!(ans);
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
