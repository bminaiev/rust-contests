//{"name":"K. Triangle","group":"Codeforces - The 2019 ICPC Asia Nanjing Regional Contest","url":"https://codeforces.com/gym/103466/problem/K","interactive":false,"timeLimit":8000,"tests":[{"input":"2\n0 0 1 1 1 0 1 0\n0 0 1 1 1 0 2 0\n","output":"0.500000000000 0.500000000000\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KTriangle"}}}

use algo_lib::f;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::polygon::PolygonT;
use algo_lib::geometry::segment::SegmentT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search_float::float_binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn triangle_area(p1: &Point, p2: &Point, p3: &Point) -> OrdF64 {
    Point::vect_mul(p1, p2, p3).abs()
}

const ITERS: usize = 60;

fn solve_input(mut a: Vec<Point>, start: Point) -> Option<Point> {
    if Point::vect_mul(&a[0], &a[1], &a[2]) < f!(0.0) {
        a.swap(1, 2);
    }
    for _ in 0..3 {
        a.rotate_left(1);
        let seg = Segment::new(a[0], a[1]);
        if !seg.contains(&start) {
            continue;
        }
        let s1 = triangle_area(&start, &a[1], &a[2]);
        let s2 = triangle_area(&start, &a[2], &a[0]);
        let sum_s = s1 + s2;
        if s1 > s2 {
            let get_point = |coef: OrdF64| -> Point { a[1] + (a[2] - a[1]).scale(coef) };
            let res = float_binary_search_first_true(f!(0.0), f!(1.0), ITERS, |coef| {
                let check = get_point(coef);
                let one_sq = triangle_area(&start, &a[1], &check);
                one_sq * f!(2.0) > sum_s
            });
            return Some(get_point(res));
        } else {
            let get_point = |coef: OrdF64| -> Point { a[2] + (a[0] - a[2]).scale(coef) };
            let res = float_binary_search_first_true(f!(0.0), f!(1.0), ITERS, |coef| {
                let check = get_point(coef);
                let one_sq = triangle_area(&start, &check, &a[0]);
                one_sq * f!(2.0) < sum_s
            });
            return Some(get_point(res));
        }
    }
    None
}

fn solve(input: &mut Input, _test_case: usize) {
    let pts = gen_vec(3, |_| {
        let x = input.read();
        let y = input.read();
        Point { x, y }
    });
    let start = Point {
        x: input.read(),
        y: input.read(),
    };
    if let Some(res) = solve_input(pts, start) {
        out_line!(res.x, res.y);
    } else {
        out_line!(-1);
    }
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
}
//END MAIN
