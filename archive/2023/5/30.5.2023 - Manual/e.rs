//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use std::collections::{HashMap, HashSet};

use algo_lib::geometry::canonical_line::CanonicalLine;
use algo_lib::geometry::orientation::{make_ccw, remove_three_on_line};
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut hm = HashMap::<_, Vec<Point>>::new();
    for _ in 0..n {
        let pts = gen_vec(3, |_| Point::new(input.i64(), input.i64()));
        for i in 0..3 {
            let p1 = pts[i];
            let p2 = pts[(i + 1) % 3];
            let line = CanonicalLine::new(&p1, &p2);
            hm.entry(line).or_default().extend([p1, p2]);
        }
    }
    let mut graph = HashMap::<_, Vec<_>>::new();
    for (_k, mut vals) in hm.into_iter() {
        vals.sort();
        for i in (0..vals.len() - 1).step_by(2) {
            let p1 = vals[i];
            let p2 = vals[i + 1];
            if p1 != p2 {
                graph.entry(p1).or_default().push(p2);
                graph.entry(p2).or_default().push(p1);
            }
        }
    }
    let mut cur = *graph.keys().next().unwrap();
    let mut seen = HashSet::new();
    let mut order = vec![];
    loop {
        order.push(cur);
        seen.insert(cur);
        let mut next = None;
        for &p in graph[&cur].iter() {
            if !seen.contains(&p) {
                next = Some(p);
                break;
            }
        }
        if let Some(p) = next {
            cur = p;
        } else {
            break;
        }
    }
    let order = make_ccw(order);
    let mut order = remove_three_on_line(order);
    let pos_of_smallest = order
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| (p.y, p.x))
        .unwrap()
        .0;
    order.rotate_left(pos_of_smallest);
    for p in order.iter() {
        out_line!(p.x, p.y);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
