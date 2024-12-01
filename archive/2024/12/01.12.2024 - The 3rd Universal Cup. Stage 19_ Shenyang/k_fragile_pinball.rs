//{"name":"K. Fragile Pinball","group":"Universal Cup - The 3rd Universal Cup. Stage 19: Shenyang","url":"https://contest.ucup.ac/contest/1865/problem/9808","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 0\n0 3\n0 -1\n","output":"5.000000000000000000\n8.000000000000000000\n8.868185038797563409\n12.210024810881955830\n"},{"input":"3\n4 0\n0 3\n0 2\n","output":"5.000000000000000000\n5.366563145999495272\n6.111919138499425171\n6.782203304416628317\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KFragilePinball"}}}

use algo_lib::collections::permutation::Permutation;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment::SegmentT;
use algo_lib::geometry::segment_intersection::inside_bounding_box;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;

type Point = PointT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn mirror_polygon(polygon: Vec<Point>, line: Line) -> Vec<Point> {
    polygon.into_iter().map(|p| line.mirror(p)).collect()
}

const MX: OrdF64 = OrdF64(1e9);
const EPS: OrdF64 = OrdF64(1e-9);

fn intersect(polygon: &[Point], line: &Line) -> (OrdF64, OrdF64) {
    let mut res = vec![];
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        if let Some(p) = line.intersect(&Line::new(&p1, &p2)) {
            if inside_bounding_box(&Segment::new(p1, p2), &p) {
                res.push(line.project(p));
            }
        }
    }
    res.sort();
    if res.is_empty() {
        (MX, -MX)
    } else {
        (res[0], *res.last().unwrap())
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| Point::new(input.f64(), input.f64()));
    let mut res = vec![OrdF64::ZERO; n + 1];
    let mut perm = Permutation::new(n);
    loop {
        let mut polygons = vec![a.clone()];
        for idx in 0..n {
            let edge_id = perm[idx];
            let prev_polygon = polygons[idx].clone();
            let line = Line::new(&prev_polygon[edge_id], &prev_polygon[(edge_id + 1) % n]);
            let new_polygon = mirror_polygon(prev_polygon, line);
            polygons.push(new_polygon);
        }
        let all_points: Vec<Point> = polygons
            .iter()
            .flat_map(|polygon| polygon.iter().copied())
            .collect();
        for &p1 in all_points.iter() {
            for &p2 in all_points.iter() {
                if p1.dist_manh(&p2) < EPS {
                    continue;
                }
                let line = Line::new(&p1, &p2);
                let intersections: Vec<_> = polygons
                    .iter()
                    .map(|polygon| intersect(polygon, &line))
                    .collect();
                if intersections[0].0 != MX {
                    let mut seg = intersections[0];
                    res[0] = res[0].max(seg.1 - seg.0);
                    for i in 1..intersections.len() {
                        if intersections[i].0 > seg.1 + EPS || intersections[i].1 < seg.1 - EPS {
                            break;
                        }
                        seg.1 = seg.1.max(intersections[i].1);
                        res[i] = res[i].max(seg.1 - seg.0);
                    }
                }
            }
        }
        if !perm.next() {
            break;
        }
    }
    let mut pref_max = OrdF64::ZERO;
    for x in res.into_iter() {
        pref_max = pref_max.max(x);
        out.println(pref_max);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "k_fragile_pinball";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
