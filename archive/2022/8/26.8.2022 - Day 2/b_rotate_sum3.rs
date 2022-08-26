//{"name":"B. Rotate Sum 3","group":"Yandex - Day 2","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39547/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n0 -1\n1 0\n0 1\n","output":"1.047197551197\n"},{"input":"3\n1 1\n4 5\n1 4\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRotateSum3"}}}

use std::cmp::{max, min};
use std::f64::consts::PI;

use algo_lib::f;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::polygon::PolygonT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::float_eq::feq;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;
type PointF = PointT<OrdF64>;

fn add_mid_points(pts: &[Point]) -> Vec<Point> {
    let mut res = vec![];
    let x0 = pts[0].x * 2;
    let y0 = pts[0].y * 2;
    for i in 0..pts.len() {
        let cur = pts[i];
        let next = pts[(i + 1) % pts.len()];
        res.push(Point::new(cur.x * 2 - x0, cur.y * 2 - y0));
        let ff = cur + next;
        res.push(Point::new(ff.x - x0, ff.y - y0));
    }
    res
}

fn solve_case(pts: &[Point]) -> f64 {
    let pts = add_mid_points(pts);
    let n = pts.len();

    let pts: Vec<_> = pts.into_iter().map(|p| p.conv_float()).collect();

    let good_sim = |start: usize| -> bool {
        let fr = pts[start];
        let to = pts[(start + n / 2) % n];

        for offset in 0..n / 2 + 1 {
            let p1 = pts[(start + offset) % n];
            let p2 = pts[(start + n - offset) % n];

            let s1 = PointF::scal_mul(&fr, &to, &p1);
            let s2 = PointF::scal_mul(&fr, &to, &p2);
            const EPS: f64 = 1e-9;
            if !feq(s1.0, s2.0, EPS) {
                return false;
            }
            let v1 = PointF::vect_mul(&fr, &to, &p1);
            let v2 = PointF::vect_mul(&fr, &to, &p2) * f!(-1.0);
            if !feq(v1.0, v2.0, EPS) {
                return false;
            }
        }
        true
    };

    let mut sim = vec![];
    for i in 0..n {
        if good_sim(i) {
            sim.push(i);
        }
        if sim.len() > 2 {
            break;
        }
    }

    if sim.is_empty() {
        return 0.0;
    }

    assert!(sim.len() != 1);

    let res = if sim.len() == 2 {
        // single simmetry
        let fr = min(sim[0], sim[1]);
        let to = max(sim[0], sim[1]);
        assert!(to - fr == n / 2);

        let line = Line::new(&pts[fr], &pts[to]);
        let mut lx1 = pts[to].x - pts[fr].x;
        let mut ly1 = pts[to].y - pts[fr].y;
        let l2 = (lx1 * lx1 + ly1 * ly1).sqrt();
        lx1 /= l2;
        ly1 /= l2;

        let mut res = 0.0;
        for i in fr..to {
            let cur = pts[i];
            let next = pts[i + 1];

            let r1_2 = line.abs_dist2(&cur);
            let r2_2 = line.abs_dist2(&next);

            let h = (lx1 * (cur.x - next.x) + ly1 * (cur.y - next.y)).abs();

            let instead_of_r2 = (r1_2 + r2_2 + r1_2.sqrt() * r2_2.sqrt()) / f!(3.0);
            res += (h * instead_of_r2).0;
        }

        res * PI
    } else {
        let center = PolygonT::new(pts.to_vec()).center_of_gravity();

        let r = pts.iter().map(|p| p.dist2(&center)).max().unwrap().sqrt().0;
        4.0 / 3.0 * PI * r * r * r
    };

    res / 8.0
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let pts = gen_vec(n, |_| Point::new(input.i64(), input.i64()));
    let res = solve_case(&pts);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
