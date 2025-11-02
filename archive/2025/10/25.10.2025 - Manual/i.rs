//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment::SegmentT;
use algo_lib::geometry::segment_intersection::inside_bounding_box;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::ord_f64::OrdF64;

type Point = PointT<OrdF64>;

const EPS: f64 = 1e-5;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _ in 0..n {
            let x = input.i64();
            let y = input.i64();
            a.push(Point::new(x as f64, y as f64));
        }
        let base = a[0];
        for i in 0..a.len() {
            a[i] = a[i] - base;
        }
        let mut mirrors = vec![a.clone()];
        for i in 0..n - 2 {
            let cur = mirrors.last().unwrap().clone();
            let p1 = cur[i + 1];
            let p2 = cur[i + 2];
            let line = Line::new(&p1, &p2);
            let mut next = vec![];
            for p in cur.iter() {
                next.push(line.mirror(*p));
            }
            mirrors.push(next);
        }
        let start = a[0];
        let final_pos = mirrors.last().unwrap()[0];
        let mut ok = start.dist_manh(&final_pos).0 > EPS;
        if ok {
            let line = Line::new(&start, &final_pos);
            let mut intersections = vec![];
            for i in 0..n - 2 {
                let p1 = mirrors[i][i + 1];
                let p2 = mirrors[i][i + 2];
                let seg = SegmentT::new(p1, p2);
                let line2 = Line::new(&p1, &p2);
                let eps = p1.dist2(&p2).0.sqrt() * EPS;
                if let Some(pt) = line.intersect(&line2) {
                    if inside_bounding_box(&seg, &pt)
                        && p1.dist2(&pt).0.sqrt() > eps
                        && p2.dist2(&pt).0.sqrt() > eps
                    {
                        intersections.push(pt);
                    } else {
                        ok = false;
                        break;
                    }
                } else {
                    ok = false;
                    break;
                }
            }
            for i in 0..intersections.len() {
                let p1 = intersections[i];
                let p2 = if i + 1 < intersections.len() {
                    intersections[i + 1]
                } else {
                    final_pos
                };
                let d1 = start.dist_manh(&p1).0;
                let d2 = start.dist_manh(&p2).0;
                if d2 < d1 * (1.0 + 1e-9) {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            out.println("YES");
            let mut angle = (final_pos.y - start.y).0.atan2((final_pos.x - start.x).0)
                / std::f64::consts::PI
                * 180.0;
            while angle < -180.0 {
                angle += 360.0;
            }
            while angle > 180.0 {
                angle -= 360.0;
            }
            out.println(angle);
        } else {
            out.println("NO");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "3");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
