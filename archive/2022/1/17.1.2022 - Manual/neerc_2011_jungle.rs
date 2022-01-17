//{"name":"neerc_2011_jungle","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"jungle.in","pattern":null},"output":{"type":"file","fileName":"jungle.out","pattern":null},"languages":{"java":{"taskClass":"neerc_2011_jungle"}}}

use algo_lib::geometry::half_plane_intersection::half_plane_intersection;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment::SegmentT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::{dbg, out, out_line};

type Point = PointT<OrdF64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut pts: Vec<Point> = input.read_vec(n);
    pts.reverse();
    let max_still_ok = binary_search_last_true(0..n, |removed| -> bool {
        let mut segs = vec![];
        for i in 0..n {
            let p1 = pts[i];
            let p2 = pts[(i + removed + 1) % pts.len()];
            segs.push(SegmentT::new(p1, p2));
        }
        half_plane_intersection(segs, None).is_some()
    })
    .unwrap();
    out_line!(max_still_ok + 1);
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
