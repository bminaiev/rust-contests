#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;

type Point = PointT<i64>;

const MAX_V: i64 = 100_000_010;
// const MAX_V: i64 = 10;

// higher X -> higher value
#[derive(Clone, Debug)]
struct ConvexHull {
    points: Vec<Point>,
}

struct ConvexHullIter {
    pos: usize,
}

impl ConvexHullIter {
    fn new() -> Self {
        Self { pos: 0 }
    }

    fn find_pos(&mut self, x: i64, hull: &ConvexHull) -> usize {
        while hull.points[self.pos].x < x {
            self.pos += 1;
        }
        while self.pos > 0 && hull.points[self.pos - 1].x >= x {
            self.pos -= 1;
        }
        self.pos
    }
}

impl ConvexHull {
    fn calc_value(&self, x: i64) -> i64 {
        let pos = binary_search_first_true(0..self.points.len(), |i| self.points[i].x >= x);
        if self.points[pos].x == x {
            return self.points[pos].y;
        }
        assert!(pos > 0);
        let left = self.points[pos - 1];
        let right = self.points[pos];
        let dx = right.x - left.x;
        let dy = right.y - left.y;
        assert!(dy % dx == 0);
        let k = dy / dx;
        left.y + k * (x - left.x)
    }

    fn calc_value_hint(&self, x: i64, iter: &mut ConvexHullIter) -> i64 {
        let pos = iter.find_pos(x, self);
        if self.points[pos].x == x {
            return self.points[pos].y;
        }
        assert!(pos > 0);
        let left = self.points[pos - 1];
        let right = self.points[pos];
        let dx = right.x - left.x;
        let dy = right.y - left.y;
        assert!(dy % dx == 0);
        let k = dy / dx;
        left.y + k * (x - left.x)
    }

    fn max_of_two(a: &ConvexHull, b: &ConvexHull) -> ConvexHull {
        let mut all_x = vec![];
        for p in &a.points {
            all_x.push(p.x);
        }
        for p in &b.points {
            all_x.push(p.x);
        }
        all_x.sort();
        all_x.dedup();
        let mut points = vec![];
        let mut a_iter = ConvexHullIter::new();
        let mut b_iter = ConvexHullIter::new();
        let mut prev_a = a.calc_value_hint(all_x[0], &mut a_iter);
        let mut prev_b = b.calc_value_hint(all_x[0], &mut b_iter);
        let mut prev_x = all_x[0];
        points.push(Point::new(all_x[0], prev_a.max(prev_b)));
        let mut a_was_better = prev_a > prev_b;
        // TODO: optimize!
        for &x in &all_x[1..] {
            let new_a = a.calc_value_hint(x, &mut a_iter);
            let new_b = b.calc_value_hint(x, &mut b_iter);

            if a_was_better != (new_a > new_b) {
                let intersection_x = if !a_was_better {
                    find_intersection_x(
                        Point::new(prev_x, prev_a),
                        Point::new(x, new_a),
                        Point::new(prev_x, prev_b),
                        Point::new(x, new_b),
                    )
                } else {
                    find_intersection_x(
                        Point::new(prev_x, prev_b),
                        Point::new(x, new_b),
                        Point::new(prev_x, prev_a),
                        Point::new(x, new_a),
                    )
                };
                if intersection_x > prev_x {
                    let intersection_y = a
                        .calc_value_hint(intersection_x, &mut a_iter)
                        .max(b.calc_value_hint(intersection_x, &mut b_iter));
                    points.push(Point::new(intersection_x, intersection_y));
                }
                if intersection_x + 1 < x {
                    let after_intersection_y = a
                        .calc_value_hint(intersection_x + 1, &mut a_iter)
                        .max(b.calc_value_hint(intersection_x + 1, &mut b_iter));
                    points.push(Point::new(intersection_x + 1, after_intersection_y));
                }
            }

            let y = new_a.max(new_b);
            points.push(Point::new(x, y));

            prev_a = new_a;
            prev_b = new_b;
            prev_x = x;
            a_was_better = new_a > new_b;
        }
        ConvexHull { points }
    }

    fn sum(a: &ConvexHull, b: &ConvexHull) -> ConvexHull {
        let mut all_x = vec![];
        for p in &a.points {
            all_x.push(p.x);
        }
        for p in &b.points {
            all_x.push(p.x);
        }
        all_x.sort();
        all_x.dedup();
        let mut points = vec![];
        let mut a_iter = ConvexHullIter::new();
        let mut b_iter = ConvexHullIter::new();
        for x in all_x {
            let y = a.calc_value_hint(x, &mut a_iter) + b.calc_value_hint(x, &mut b_iter);
            points.push(Point::new(x, y));
        }
        ConvexHull { points }
    }
}

fn find_intersection_x(a_left: Point, a_right: Point, b_left: Point, b_right: Point) -> i64 {
    assert!(a_left.y <= b_left.y);
    assert!(a_left.x == b_left.x);
    assert!(a_right.x == b_right.x);
    assert!(a_right.y >= b_right.y);

    let dx = a_right.x - a_left.x;
    let dy_a = a_right.y - a_left.y;
    let dy_b = b_right.y - b_left.y;
    assert!(dy_a % dx == 0);
    assert!(dy_b % dx == 0);
    let k_a = dy_a / dx;
    let k_b = dy_b / dx;
    let k_diff = k_a - k_b;
    assert!(k_diff > 0);
    let y_diff = b_left.y - a_left.y;
    let need_x = y_diff / k_diff;
    a_left.x + need_x
}

struct Node {
    prefix: ConvexHull,
    suffix: ConvexHull,
    full: ConvexHull,
    best: ConvexHull,
}

impl Node {
    fn single(value: i64) -> Self {
        let p1 = Point::new(-MAX_V, value - MAX_V);
        let p2 = Point::new(MAX_V, value + MAX_V);
        let ch = ConvexHull {
            points: vec![p1, p2],
        };
        Self {
            prefix: ch.clone(),
            suffix: ch.clone(),
            full: ch.clone(),
            best: ch,
        }
    }

    fn merge(left: &Node, right: &Node) -> Self {
        let prefix =
            ConvexHull::max_of_two(&left.prefix, &ConvexHull::sum(&left.full, &right.prefix));
        let suffix =
            ConvexHull::max_of_two(&right.suffix, &ConvexHull::sum(&right.full, &left.suffix));
        let best = ConvexHull::max_of_two(
            &ConvexHull::sum(&left.suffix, &right.prefix),
            &ConvexHull::max_of_two(&left.best, &right.best),
        );
        Self {
            prefix,
            suffix,
            full: ConvexHull::sum(&left.full, &right.full),
            best,
        }
    }
}

fn calc(a: &[i64]) -> Node {
    if a.len() == 1 {
        Node::single(a[0])
    } else {
        let mid = a.len() / 2;
        let left = calc(&a[..mid]);
        let right = calc(&a[mid..]);
        Node::merge(&left, &right)
    }
}

fn solve_fast(a: &[i64], q: &[i64]) -> Vec<i64> {
    let mut res = vec![];
    let node = calc(a);
    // dbg!(node.prefix);
    // dbg!(node.suffix);
    // dbg!(node.best);
    for q in q {
        res.push(node.best.calc_value(*q));
    }
    res
}

fn solve_slow(a: &[i64], q: &[i64]) -> Vec<i64> {
    let mut res = vec![];
    for &x in q {
        let mut best = i64::MIN;
        for i in 0..a.len() {
            for j in i..a.len() {
                let mut sum = 0;
                for k in i..=j {
                    sum += a[k];
                }
                let value = sum + (j - i + 1) as i64 * x;
                best = best.max(value);
            }
        }
        res.push(best);
    }
    res
}

fn stress() {
    for it in 17.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..100);
        const MAX_VALUE: i64 = 100;
        let a = rnd.gen_vec(n, -MAX_VALUE..MAX_VALUE);
        // let q = vec![rnd.gen_range(-MAX_VALUE..MAX_VALUE)];
        let q = rnd.gen_vec(n, -MAX_VALUE..MAX_VALUE);
        // dbg!(a, q);
        let res_fast = solve_fast(&a, &q);
        let res_slow = solve_slow(&a, &q);
        if res_fast != res_slow {
            dbg!(a);
            dbg!(q);
            dbg!(res_fast);
            dbg!(res_slow);
            break;
        }
    }
}

fn stress2() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = 200_000;
        const MAX_VALUE: i64 = 100_000_000;
        let a = rnd.gen_vec(n, -MAX_VALUE..MAX_VALUE);
        let q = rnd.gen_vec(n, -MAX_VALUE..MAX_VALUE);
        let instant = std::time::Instant::now();
        let res_fast = solve_fast(&a, &q);
        dbg!("done!", instant.elapsed());
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let q = input.usize();
        let a = input.vec::<i64>(n);
        let queries = input.vec::<i64>(q);
        let res = solve_fast(&a, &queries);
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e";
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
