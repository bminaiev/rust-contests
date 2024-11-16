//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use std::collections::HashSet;
use std::i64::MAX;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::convex_hull::convex_hull;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::polygon::PolygonT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

// 10:34
type Point = PointT<i64>;

fn solve_case(a: &[Point], slow: bool) -> i64 {
    let hull = convex_hull(a);
    let in_hull: HashSet<Point> = hull.iter().cloned().collect();
    let mut not_in_hull = vec![];
    for &p in a.iter() {
        if !in_hull.contains(&p) {
            not_in_hull.push(p);
        }
    }
    if not_in_hull.is_empty() {
        return -1;
    }
    let full_area = PolygonT::new(hull.clone()).area_x2();
    let mut smallest_remove = i64::MAX;

    if slow {
        for i in 0..hull.len() {
            let p = hull[i];
            let q = hull[(i + 1) % hull.len()];
            for x in not_in_hull.iter() {
                let area = Point::vect_mul(&p, &q, x).abs();
                smallest_remove = smallest_remove.min(area);
            }
        }
        return full_area - smallest_remove;
    }

    let mut inside_hull = convex_hull(&not_in_hull);
    assert!(!inside_hull.is_empty());
    inside_hull.push(inside_hull[0]);

    let mut mid = 0;
    for i in 0..inside_hull.len() - 1 {
        if inside_hull[i] > inside_hull[mid] {
            mid = i;
        }
    }

    let first = &inside_hull[..mid + 1];
    let second = &inside_hull[mid..];

    let score_seg = |p: Point, q: Point, line: &[Point]| -> i64 {
        if slow {
            let mut res = i64::MAX;
            for x in line.iter() {
                let area = Point::vect_mul(&p, &q, x).abs();
                res = res.min(area);
            }
            res
        } else {
            let pos = binary_search_first_true(0..line.len() - 1, |i| {
                let area1 = Point::vect_mul(&p, &q, &line[i]).abs();
                let area2 = Point::vect_mul(&p, &q, &line[i + 1]).abs();
                area2 > area1
            });
            Point::vect_mul(&p, &q, &line[pos]).abs()
        }
    };

    for i in 0..hull.len() {
        let p = hull[i];
        let q = hull[(i + 1) % hull.len()];
        let score_first = score_seg(p, q, first);
        let score_second = score_seg(p, q, second);
        let score = score_first.min(score_second);
        smallest_remove = smallest_remove.min(score);
    }

    full_area - smallest_remove
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| Point::new(input.i64(), input.i64()));
    let res = solve_case(&a, false);
    out.println(res);
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX: usize = 10;
        const MAX_C: i64 = 5;
        let n = rnd.gen(1..MAX);
        let mut a = vec![];
        for _ in 0..n {
            let x = rnd.gen(0..MAX_C);
            let y = rnd.gen(0..MAX_C);
            a.push(Point::new(x, y));
        }
        a.sort();
        a.dedup();
        rnd.shuffle(&mut a);
        let slow_ans = solve_case(&a, true);
        let fast_ans = solve_case(&a, false);
        dbg!(slow_ans);
        if slow_ans != fast_ans {
            dbg!(a);
            dbg!(slow_ans, fast_ans);
            panic!();
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
