//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type Point = PointT<i32>;

type State = HashMap<Point, usize>;

const DEST: Point = Point { x: 0, y: 6 };

fn cost(s: &State) -> Vec<i32> {
    let mut res = vec![];
    for (p, &v) in s.iter() {
        let d = p.dist_manh(&DEST);
        for _ in 0..v {
            res.push(d);
        }
    }
    res.sort();
    res
}

fn add(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn transitions(s: &State, shifts: &[[Point; 2]]) -> Vec<State> {
    let mut res = vec![];
    let mut all_points = vec![];
    for (&p, &cnt) in s.iter() {
        if cnt != 0 {
            all_points.push(p);
        }
    }
    all_points.sort_by_key(|p| p.dist_manh(&DEST));
    // all_points.reverse();
    for &p in all_points.iter() {
        let cnt = s[&p];
        for shift in shifts.iter() {
            let p2 = add(p, shift[0]);
            let p3 = add(p2, shift[1]);
            if let Some(cnt_p2) = s.get(&p2) {
                if *cnt_p2 > 0 {
                    let mut ns = s.clone();
                    ns.insert(p, cnt - 1);
                    *ns.entry(p2).or_default() -= 1;
                    *ns.entry(p3).or_insert(0) += 1;
                    res.push(ns);
                }
            }
        }
        if res.len() > 100 {
            break;
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    const M: i32 = 35;
    let mut shifts = vec![];
    for dx in -1..=1i32 {
        for dy in -1..=1i32 {
            if dx.abs() + dy.abs() == 1 {
                for &mult in [-1, 1].iter() {
                    let cur = [Point::new(dx, dy), Point::new(dy * mult, dx * mult)];
                    shifts.push(cur);
                }
            }
        }
    }
    let mut start_state = HashMap::new();
    for x in -M..=M {
        for y in -M..=0 {
            let p = Point::new(x, y);
            start_state.insert(p, 1);
        }
    }
    let mut state = start_state;
    let mut iter = 0;
    loop {
        iter += 1;
        let score = cost(&state);
        dbg!(iter, score[..10]);
        let tr = transitions(&state, &shifts);
        let best_tr = tr.iter().min_by_key(|s| cost(s));
        if let Some(best) = best_tr {
            state = best.clone();
        } else {
            dbg!("BREAK");
            break;
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
