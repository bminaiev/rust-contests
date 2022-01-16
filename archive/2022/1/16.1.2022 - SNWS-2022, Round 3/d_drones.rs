//{"name":"D. Drones","group":"Yandex - SNWS-2022, Round 3","url":"https://contest.yandex.ru/snws2022/contest/23959/problems/D/","interactive":false,"timeLimit":2000,"tests":[{"input":"1 1\n20 22\n1 1 1 1 1\n3 4\n20 150\n100 10\n180 150\n100 100 20 20 3\n80 180 2 2 4\n20 40 20 30 2\n100 140 100 100 5\n4 2\n5 5\n5 15\n15 15\n15 5\n10 10 2 2 2\n10 10 3 3 3\n0 0\n","output":"1.000000\n7.357143\n1.250000\n"}],"testType":"multiEof","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDrones"}}}

use algo_lib::geometry::half_plane_intersection::half_plane_intersection;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment::SegmentT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::{dbg, out, out_line};

type Point = PointT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn solve(input: &mut Input, _test_case: usize) {
    let num_drones = input.usize();
    let num_queries = input.usize();
    if num_drones == 0 {
        return;
    }
    let drones: Vec<Point> = input.read_vec(num_drones);
    let mut res = vec![OrdF64::ZERO; num_drones];
    for _ in 0..num_queries {
        let center: Point = input.read();
        let dx: OrdF64 = input.read();
        let dy: OrdF64 = input.read();
        let bonus = input.read();
        let bottom_left = center.shift(-dx, -dy);
        let bottom_right = center.shift(dx, -dy);
        let top_right = center.shift(dx, dy);
        let top_left = center.shift(-dx, dy);

        let rect_area = dx * dy * OrdF64(4.0);

        for i in 0..num_drones {
            let mut segs = vec![
                Segment::new(bottom_left, bottom_right),
                Segment::new(bottom_right, top_right),
                Segment::new(top_right, top_left),
                Segment::new(top_left, bottom_left),
            ];
            for j in 0..num_drones {
                if j == i {
                    continue;
                }
                let pi = drones[i];
                let pj = drones[j];
                let mid = (pi + pj).scale(OrdF64(0.5));
                let dir = (pj - pi).rotate_ccw();
                segs.push(Segment::new(mid, mid + dir));
            }
            if let Some(poly) = half_plane_intersection(segs, None) {
                res[i] += poly.area() * bonus / rect_area;
            }
        }
    }
    out_line!(*res.iter().max().unwrap());
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut i = 1usize;
    while input.peek().is_some() {
        solve(&mut input, i);
        i += 1;
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
