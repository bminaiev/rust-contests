#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

use algo_lib::geometry::segment::SegmentT;
use algo_lib::geometry::segment_intersection_dd::segment_intersection_dd;
use algo_lib::misc::double_double::DoubleDouble;

type Point = PointT<DoubleDouble>;

fn seg_sum(p1: Point, p2: Point) -> DoubleDouble {
    p1.x * p2.y - p1.y * p2.x
}

fn solve_case(a: &[Point]) -> f64 {
    let n = a.len();
    if n < 6 {
        return 0.0;
    }
    let get_p = |base: usize, offset: i32| -> Point {
        let i = ((base + 2 * n) as i32 + offset) % (n as i32);
        a[i as usize]
    };
    let get = |base: usize, offsets: [i32; 4]| -> PointT<DoubleDouble> {
        let p0 = get_p(base, offsets[0]);
        let p1 = get_p(base, offsets[1]);
        let p2 = get_p(base, offsets[2]);
        let p3 = get_p(base, offsets[3]);
        let seg1 = SegmentT::new(p0, p1);
        let seg2 = SegmentT::new(p2, p3);
        let inter = segment_intersection_dd(&seg1, &seg2);
        inter.unwrap()
    };
    let get_def = |i: usize| -> DoubleDouble {
        let p0 = get(i, [-2, 0, -1, 1]);
        let p1 = get(i, [-1, 1, 0, 2]);
        seg_sum(p0, p1)
    };
    let mut sum = DoubleDouble::ZERO;
    for i in 0..n {
        sum += get_def(i);
    }
    assert!(sum >= DoubleDouble::ZERO);
    if n % 2 == 0 {
        return sum.to_f64();
    }
    let mut res = DoubleDouble::NEG_INFINITY;
    for base in 0..n {
        // base+1 and base+2 <- same color
        let mut cur_sum = sum;
        for i in 0..4 {
            cur_sum -= get_def(base + i);
        }
        let p0 = get(base, [-2, 0, -1, 1]);
        let p1 = get(base, [-1, 1, 0, 3]);
        let p2 = get(base, [0, 3, 2, 4]);
        let p3 = get(base, [2, 4, 3, 5]);
        cur_sum += seg_sum(p0, p1);
        cur_sum += seg_sum(p1, p2);
        cur_sum += seg_sum(p2, p3);
        if cur_sum > res {
            res = cur_sum;
        }
    }
    res.to_f64()
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for i in 0..n {
            a.push(Point::new(input.i64(), input.i64()));
        }
        let res = solve_case(&a);
        out.println(res / 2.0);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "l_white_night_3";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
