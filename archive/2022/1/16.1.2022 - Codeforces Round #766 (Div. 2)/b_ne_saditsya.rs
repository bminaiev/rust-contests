//{"name":"B. Не садиться","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 3\n1 2\n","output":"3 3 4 4 4 4 4 4 5 5 5 5\n1 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNeSaditsya"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let m = input.i32();
    let mut corners = vec![
        Point::new(0, 0),
        Point::new(n - 1, 0),
        Point::new(0, m - 1),
        Point::new(n - 1, m - 1),
    ];
    let mut dists = vec![];
    for x in 0..n {
        for y in 0..m {
            let p = Point::new(x, y);
            let mut max_dist = 0;
            for p2 in corners.iter() {
                let dist = (p2.x - p.x).abs() + (p2.y - p.y).abs();
                max_dist.update_max(dist);
            }
            dists.push(max_dist);
        }
    }
    dists.sort();
    out_line!(dists);
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
