//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use std::time::Instant;

use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

trait Interactor {
    fn ask(&mut self, p1: Point, p2: Point) -> f64;
}

struct MyInteractor {
    more_queries: usize,
    p1: Point,
    p2: Point,
}

impl MyInteractor {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self {
            more_queries: 25_000,
            p1,
            p2,
        }
    }
}

struct RealInteractor<'a> {
    input: &'a mut Input,
}

impl<'a> RealInteractor<'a> {
    pub fn new(input: &'a mut Input) -> Self {
        Self { input }
    }
}

impl<'a> Interactor for RealInteractor<'a> {
    fn ask(&mut self, p1: Point, p2: Point) -> f64 {
        out_line!(p1.x, p1.y, p2.x, p2.y);
        output().flush();
        self.input.f64()
    }
}

fn calc_res(p1: Point, p2: Point, query: &[Point]) -> f64 {
    let s1 = (p1.dist2(&query[0]) as f64).sqrt() + (p2.dist2(&query[1]) as f64).sqrt();
    let s2 = (p1.dist2(&query[1]) as f64).sqrt() + (p2.dist2(&query[0]) as f64).sqrt();
    let s = f64::min(s1, s2);

    let mut sim = f64::max(0.0, (40000.0 - s) / 40000.0);
    sim *= 1000.0;
    let r = sim.trunc() / 10.0;
    // dbg!(s, r);
    r
}

impl Interactor for MyInteractor {
    fn ask(&mut self, p1: Point, p2: Point) -> f64 {
        assert!(self.more_queries > 0);
        // dbg!(self.more_queries);
        self.more_queries -= 1;
        calc_res(self.p1, self.p2, &[p1, p2])
    }
}

const MX: i64 = 10_000;
fn is_ok_p(p: Point) -> bool {
    p.x.abs() <= MX && p.y.abs() <= MX
}

fn is_ok(pts: &[Point]) -> bool {
    for &p in pts.iter() {
        if !is_ok_p(p) {
            return false;
        }
    }
    pts[0] != pts[1]
}

#[derive(Clone)]
struct SeenQuery {
    pts: Vec<Point>,
    sim: f64,
}

fn solve_case(inter: &mut impl Interactor) {
    let start = Instant::now();
    let mut pts = vec![Point::new(123, 0), Point::new(0, -99)];
    let mut cur = inter.ask(pts[0], pts[1]);
    let mut shifts = vec![];
    shifts.push(Point::new(1, 0));
    shifts.push(Point::new(0, 1));
    shifts.push(Point::new(-1, 0));
    shifts.push(Point::new(0, -1));

    let mut seen_queries = vec![];

    let mut rnd = Random::new(787788);

    for &step in [150, 105, 65, 44, 33, 22].iter() {
        loop {
            let mut changed = false;
            for it in 0..2 {
                for shift in shifts.iter() {
                    let new_p = pts[it] + shift.scale(step);
                    let mut new_pts = pts.clone();
                    new_pts[it] = new_p;
                    if !is_ok(&new_pts) {
                        continue;
                    }
                    let new_sim = inter.ask(new_pts[0], new_pts[1]);
                    if new_sim == 100.0 {
                        return;
                    }
                    seen_queries.push(SeenQuery {
                        pts: new_pts,
                        sim: new_sim,
                    });
                    if new_sim > cur {
                        pts[it] = new_p;
                        cur = new_sim;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        loop {
            let mut changed = false;
            const M: usize = 99;
            for angle_it in 0..M {
                let angle = (angle_it as f64 / M as f64) * 2.0 * std::f64::consts::PI;
                for it in 0..2 {
                    let shift = Point::new(
                        ((step as f64) * angle.cos()) as i64,
                        ((step as f64) * angle.sin()) as i64,
                    );
                    let new_p = pts[it] + shift;
                    let mut new_pts = pts.clone();
                    new_pts[it] = new_p;
                    if !is_ok(&new_pts) {
                        continue;
                    }
                    let new_sim = inter.ask(new_pts[0], new_pts[1]);
                    if new_sim == 100.0 {
                        return;
                    }
                    seen_queries.push(SeenQuery {
                        pts: new_pts,
                        sim: new_sim,
                    });
                    if new_sim > cur {
                        pts[it] = new_p;
                        cur = new_sim;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }

    seen_queries.reverse();

    dbg!("1", start.elapsed());

    // TODO:!!!
    dbg!(cur);
    while cur == 99.8 {
        let step = rnd.gen(20..60);
        let angle1 = rnd.gen_double() * 2.0 * std::f64::consts::PI;
        let angle2: f64 = rnd.gen_double() * 2.0 * std::f64::consts::PI;
        let shift1 = Point::new(
            ((step as f64) * angle1.cos()) as i64,
            ((step as f64) * angle1.sin()) as i64,
        );
        let shift2 = Point::new(
            ((step as f64) * angle2.cos()) as i64,
            ((step as f64) * angle2.sin()) as i64,
        );
        let new_p1 = pts[0] + shift1;
        let new_p2: PointT<i64> = pts[1] + shift2;
        if !is_ok(&[new_p1, new_p2]) {
            continue;
        }
        let res = inter.ask(new_p1, new_p2);
        if res == 100.0 {
            return;
        }
        seen_queries.push(SeenQuery {
            pts: vec![new_p1, new_p2],
            sim: res,
        });
        if res > cur {
            pts[0] = new_p1;
            pts[1] = new_p2;
            cur = res;
            break;
        }
    }
    dbg!("2", start.elapsed());
    assert!(cur >= 99.9, "{:?}", pts);

    let delta = 40;

    let n1 = near(pts[0], delta);
    let n2 = near(pts[1], delta);

    let mut candidates = vec![];

    let mut itt = 0;
    let mut sum = 0;
    for &p1 in n1.iter() {
        for &p2 in n2.iter() {
            if (p1.dist2(&pts[0]) as f64).sqrt() + (p2.dist2(&pts[1]) as f64).sqrt() > delta as f64
            {
                break;
            }
            itt += 1;
            let mut ok = true;
            let mut good_q = None;
            for i in (0..seen_queries.len()).rev() {
                let sq = &seen_queries[i];
                let res = calc_res(p1, p2, &sq.pts);
                sum += 1;
                if res != sq.sim {
                    ok = false;
                    good_q = Some(i);
                    break;
                }
            }
            if ok {
                candidates.push([p1, p2]);
            } else {
                let good_q = good_q.unwrap();
                let query = seen_queries.remove(good_q);
                seen_queries.push(query);
            }
        }
    }
    dbg!(sum);
    dbg!(itt);
    dbg!(candidates.len());
    dbg!("3", start.elapsed());
    assert!(!candidates.is_empty());
    while candidates.len() > 1 {
        let p1 = Point::new(rnd.gen(-MX..MX), rnd.gen(-MX..MX));
        let p2 = Point::new(rnd.gen(-MX..MX), rnd.gen(-MX..MX));
        let res = inter.ask(p1, p2);
        if res == 100.0 {
            return;
        }
        let mut new_candidates = vec![];
        for cand in candidates.iter() {
            let res2 = calc_res(p1, p2, cand);
            if res == res2 {
                new_candidates.push(*cand);
            }
        }
        candidates = new_candidates;
    }
    dbg!("4", start.elapsed());
    let c = candidates[0];
    let r = inter.ask(c[0], c[1]);
    assert!(r == 100.0);
}

fn near(p: Point, delta: i64) -> Vec<Point> {
    let mut res = vec![];
    for dx in -delta..=delta {
        for dy in -delta..=delta {
            if dx * dx + dy * dy > delta * delta {
                continue;
            }
            let p = Point::new(p.x + dx, p.y + dy);
            if is_ok_p(p) {
                res.push(p);
            }
        }
    }
    res.sort_by_key(|p2| p2.dist2(&p));
    res
}

fn stress() {
    for it in 136.. {
        dbg!(it);
        let start = Instant::now();
        let mut rnd = Random::new(it);
        let p1 = Point::new(rnd.gen(-MX..MX), rnd.gen(-MX..MX));
        let p2 = Point::new(rnd.gen(-MX..MX), rnd.gen(-MX..MX));
        if p1.dist2(&p2) < 10_000 {
            continue;
        }
        dbg!("test", p1, p2);
        let mut inter = MyInteractor::new(p1, p2);
        solve_case(&mut inter);
        dbg!(start.elapsed());
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut inter = RealInteractor::new(input);
    solve_case(&mut inter);
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
