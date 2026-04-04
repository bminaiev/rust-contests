//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::convex_hull::convex_hull;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

type Point = PointT<i64>;

fn check_order(order_p: &[Point]) {
    // for p in order_p.iter() {
    //     dbg!(p);
    // }
    for i in 0..order_p.len() - 2 {
        let p1 = order_p[i];
        let p2 = order_p[i + 1];
        let p3 = order_p[i + 2];
        let vmul = Point::vect_mul(&p1, &p2, &p3);
        assert!(vmul >= 0);
    }
}

fn solve_case(mut a: Vec<Point>) -> Vec<usize> {
    let mut hm = HashMap::new();
    for i in 0..a.len() {
        let p = a[i];
        hm.insert(p, i);
    }
    let mut order = vec![];
    let mut order_p = vec![];
    while !a.is_empty() {
        let ch = convex_hull(&a);
        for p in ch {
            let id = hm[&p];
            order.push(id + 1);
            order_p.push(p);
            hm.remove(&p);
        }
        a.retain(|p| hm.contains_key(p));
    }
    check_order(&order_p);
    order
}

fn solve_correct(mut a: Vec<Point>) -> Vec<usize> {
    let mut seen = vec![false; a.len()];
    let mut last = Point::new(0, -1);
    let mut order = vec![];
    let mut order_p = vec![];
    for _ in 0..a.len() {
        let mut best = None;
        for i in 0..a.len() {
            if seen[i] {
                continue;
            }
            if best.is_none() {
                best = Some(i);
            } else {
                let vmul = Point::vect_mul(&last, &a[best.unwrap()], &a[i]);
                if vmul > 0 {
                    continue;
                }
                if vmul < 0 {
                    best = Some(i);
                } else {
                    if a[best.unwrap()].dist2(&last) > a[i].dist2(&last) {
                        best = Some(i);
                    }
                }
            }
        }
        let b = best.unwrap();
        order.push(b + 1);
        order_p.push(a[b]);
        last = a[b];
        seen[b] = true;
    }
    check_order(&order_p);
    order
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _ in 0..n {
            let p = Point::new(input.i64(), input.i64());
            a.push(p);
        }
        let order = solve_correct(a);
        out.println(order);
    }
}

fn stress() {
    for it in 49.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(3..40);
        let mut a = vec![];
        let mut seen = HashMap::new();
        const MAX_X: i64 = 30;
        while seen.len() < n {
            let p = Point::new(rnd.gen_range(0..MAX_X), rnd.gen_range(0..MAX_X));
            if !seen.contains_key(&p) {
                seen.insert(p, true);
                a.push(p);
            }
        }
        let order = solve_correct(a.clone());
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
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
