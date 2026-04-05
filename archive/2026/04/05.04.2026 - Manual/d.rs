//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

type Point = PointT<i64>;

#[derive(Clone)]
struct Segm {
    shift: i64,
    start: Point,
    end: Point,
}

struct ByCoord {
    segs: HashMap<i64, Vec<Segm>>,
}

fn dist(start: Point, end: Point, q: Point) -> Option<i64> {
    if q.x >= start.x.min(end.x)
        && q.x <= start.x.max(end.x)
        && q.y >= start.y.min(end.y)
        && q.y <= start.y.max(end.y)
    {
        Some(start.dist_manh(&q))
    } else {
        None
    }
}

impl ByCoord {
    fn new() -> Self {
        Self {
            segs: HashMap::new(),
        }
    }

    fn add(&mut self, shift: i64, start: Point, end: Point) {
        assert_eq!(start.x, end.x);
        let seg = Segm { shift, start, end };
        self.segs.entry(start.x).or_default().push(seg);
    }

    fn build(&mut self) {
        for (k, v) in self.segs.iter_mut() {
            v.sort_by_key(|p| p.start.y);
        }
    }

    fn find_pos(&self, p: Point) -> Option<i64> {
        if let Some(vals) = self.segs.get(&p.x) {
            let idx = binary_search_first_true(0..vals.len(), |i| {
                let max_y = vals[i].end.y.max(vals[i].start.y);
                max_y >= p.y
            });
            if idx == vals.len() {
                None
            } else {
                dist(vals[idx].start, vals[idx].end, p).map(|d| d + vals[idx].shift)
            }
        } else {
            None
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let mut a = vec![];
    for _ in 0..n {
        let x = input.i64();
        let y = input.i64();
        a.push(Point::new(x, y));
    }
    let mut finds = vec![];
    for _ in 0..m {
        let x = input.i64();
        let y = input.i64();
        finds.push(Point::new(x, y));
    }
    let mut by_x = ByCoord::new();
    let mut by_y = ByCoord::new();
    let mut sum = 0;
    for i in 0..n {
        let cur = a[i];
        let next = a[(i + 1) % n];
        if cur.x == next.x {
            by_x.add(sum, cur, next);
        } else {
            by_y.add(sum, Point::new(cur.y, cur.x), Point::new(next.y, next.x));
        }
        sum += cur.dist_manh(&next);
    }
    by_x.build();
    by_y.build();
    let mut positions = vec![];
    for find in finds {
        if let Some(pos) = by_x.find_pos(find) {
            positions.push(pos);
        } else if let Some(pos) = by_y.find_pos(Point::new(find.y, find.x)) {
            positions.push(pos);
        } else {
            unreachable!();
        }
    }
    positions.sort();
    let mut max_dist = 0;
    for i in 0..m {
        let next = (i + 1) % m;
        let dist = (positions[next] + sum - positions[i]) % sum;
        max_dist = max_dist.max(dist);
    }
    if m == 1 {
        max_dist = sum;
    }
    out.println(sum - max_dist);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d";
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
