//{"name":"B. Безопасная передача информации","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/B/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n8\n3 1\n1 3\n-1 3\n-3 1\n-3 -1\n-1 -3\n1 -3\n3 -1\n4\n2 2\n-1 2\n-1 -1\n2 1\n3\n4 3\n0 0\n7 0\n4\n1 3 2 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBezopasnayaPeredachaInformatsii"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::polygon::PolygonT;
use algo_lib::geometry::segment::SegmentT;
use algo_lib::geometry::segment_intersection::segment_intersection;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::misc::ternary_search::ternary_search_find_max;
use algo_lib::{dbg, out, out_line};

type Point = PointT<OrdF64>;
type Polygon = PolygonT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn max_radius(p: &Point, poly: &Polygon) -> OrdF64 {
    let mut res = OrdF64::MAX;
    for edge in poly.edges() {
        res.update_min(edge.to_line().abs_dist(p));
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let polygons: Vec<Polygon> = input.read_vec(n);
    let cnt_routers = input.usize();
    let mut routers = input.read_vec::<OrdF64>(cnt_routers);
    routers.sort();
    const ITERS: usize = 100;
    const MAX_COORD: OrdF64 = OrdF64(1e6);
    let biggest_radius = |poly: &Polygon| -> OrdF64 {
        let min_x = poly.min_x();
        let max_x = poly.max_x();
        let res = ternary_search_find_max(min_x, max_x, ITERS, |x| {
            let vertical = Segment::new(Point::new(x, -MAX_COORD), Point::new(x, MAX_COORD));
            let mut intersections = vec![];
            for edge in poly.edges() {
                if let Some(inter) = segment_intersection(&edge, &vertical) {
                    intersections.push(inter);
                }
            }
            intersections.sort_by_key(|p| p.y);
            assert!(intersections.len() >= 2);
            let min_y = intersections[0].y;
            let max_y = intersections.last_exn().y;
            ternary_search_find_max(min_y, max_y, ITERS, |y| max_radius(&Point::new(x, y), poly))
        });
        res.value.value
    };
    let mut can_fit: Vec<_> = polygons.iter().map(biggest_radius).collect();
    can_fit.sort();
    let mut res = 0;
    let mut iter = 0;
    for &router in routers.iter() {
        while iter != can_fit.len() && can_fit[iter] < router {
            iter += 1;
        }
        if iter != can_fit.len() {
            iter += 1;
            res += 1;
        }
    }
    out_line!(res);
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
