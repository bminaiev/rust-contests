use algo_lib::collections::fx_hash_map::FxHashMap;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::nearest_points::find_nearest_points;
use algo_lib::geometry::point::{PointT, PointWithIdT};
use algo_lib::geometry::range_tree::RangeTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::misc::rand::Random;

type Point = PointT<i64>;
type PointD = PointT<OrdF64>;

#[derive(Clone, Copy)]
struct PointWithMass {
    point: Point,
    mass: usize,
    id: usize,
}

fn solve_stationary(mut points: Vec<PointWithMass>, res: &mut [f64]) {
    if points.len() <= 1 {
        return;
    }
    points.sort_by_key(|p| p.mass);
    let mut seen = FxHashMap::<Point, usize>::default();
    let mut n_points = vec![];
    for p in points.iter() {
        if let Some(id) = seen.get(&p.point) {
            res[p.id] = 0.0;
            res[*id] = 0.0;
            continue;
        }
        seen.insert(p.point, p.id);
        n_points.push(*p);
    }

    let nearby_points_input = n_points.iter().map(|p| p.point).collect::<Vec<_>>();
    let nearby_points = find_nearest_points(&nearby_points_input);

    let range_tree_points = n_points
        .iter()
        .enumerate()
        .map(|(i, p)| PointWithIdT::new(p.point, i))
        .collect::<Vec<_>>();
    let range_tree = RangeTree::new(range_tree_points);

    for i in 0..n_points.len() {
        let p = n_points[i];
        if res[p.id] == 0.0 {
            continue;
        }
        let nearest_id = nearby_points[i];
        let p2 = n_points[nearest_id];
        let dist = (p.point.dist2(&p2.point) as f64).sqrt() as i64 + 2;
        let max_dist = dist * 10;
        let xs = p.point.x - max_dist..p.point.x + max_dist;
        let ys = p.point.y - max_dist..p.point.y + max_dist;

        let mut smallest_result = i64::MAX;

        range_tree.iter_rect(xs, ys, |points: &[PointWithIdT<i64>]| {
            for p2 in points.iter() {
                if p2.id() == i {
                    continue;
                }
                let result = p2.p.dist2(&p.point) * n_points[p2.id()].mass as i64;
                if result < smallest_result {
                    smallest_result = result;
                }
            }
        });

        let cur_smallest_result = (smallest_result as f64) * (p.mass as f64);
        if cur_smallest_result < res[p.id] {
            res[p.id] = cur_smallest_result;
        }
    }
}

fn moving_key(p: &Point, dx: i64, dy: i64) -> i128 {
    dx as i128 * p.y as i128 - dy as i128 * p.x as i128
}

#[derive(Clone, Copy)]
struct MovingPoint {
    x: i128,
    mass: usize,
    id: usize,
}

#[derive(Clone, Copy)]
struct MovingEvent {
    x: i128,
    kind: u8,
    point: MovingPoint,
}

struct SweepCandidate {
    x: f64,
    mass: usize,
    slope: f64,
    until: f64,
}

fn add_sweep_candidate(stack: &mut Vec<SweepCandidate>, point: MovingPoint) {
    const INF: f64 = 1e100;
    let x = point.x as f64;
    let slope = (point.mass as f64).sqrt();
    while let Some(last) = stack.last() {
        let last_value = (last.until - last.x) * last.slope;
        let new_value = (last.until - x) * slope;
        if last.mass >= point.mass || last_value >= new_value {
            stack.pop();
        } else {
            break;
        }
    }
    let until = stack.last().map_or(INF, |last| {
        (x * slope - last.x * last.slope) / (slope - last.slope)
    });
    stack.push(SweepCandidate {
        x,
        mass: point.mass,
        slope,
        until,
    });
}

fn update_moving_half(
    queries: &[MovingPoint],
    candidates: &[MovingPoint],
    sign: i32,
    denom: f64,
    res: &mut [f64],
) {
    let sign = sign as i128;
    let mut events = Vec::with_capacity(queries.len() + candidates.len());
    for &point in candidates.iter() {
        let x = point.x * sign;
        events.push(MovingEvent {
            x,
            kind: 0,
            point: MovingPoint { x, ..point },
        });
    }
    for &point in queries.iter() {
        let x = point.x * sign;
        events.push(MovingEvent {
            x,
            kind: 1,
            point: MovingPoint { x, ..point },
        });
    }
    events.sort_by_key(|event| (event.x, event.kind));

    let mut stack = Vec::<SweepCandidate>::new();
    for event in events {
        let x = event.x as f64;
        while stack.last().is_some_and(|candidate| candidate.until < x) {
            stack.pop();
        }
        if event.kind == 0 {
            add_sweep_candidate(&mut stack, event.point);
        } else if let Some(candidate) = stack.last() {
            let dist = x - candidate.x;
            let cur_res = dist * dist * (candidate.mass as f64) * (event.point.mass as f64) / denom;
            if cur_res < res[event.point.id] {
                res[event.point.id] = cur_res;
            }
        }
    }
}

fn update_moving(
    queries: &[PointWithMass],
    candidates: &[PointWithMass],
    dx: i64,
    dy: i64,
    res: &mut [f64],
) {
    if queries.is_empty() || candidates.is_empty() {
        return;
    }
    let queries = queries
        .iter()
        .map(|p| MovingPoint {
            x: moving_key(&p.point, dx, dy),
            mass: p.mass,
            id: p.id,
        })
        .collect::<Vec<_>>();
    let candidates = candidates
        .iter()
        .map(|p| MovingPoint {
            x: moving_key(&p.point, dx, dy),
            mass: p.mass,
            id: p.id,
        })
        .collect::<Vec<_>>();
    let denom = (dx * dx + dy * dy) as f64;
    update_moving_half(&queries, &candidates, 1, denom, res);
    update_moving_half(&queries, &candidates, -1, denom, res);
}

fn solve_moving(
    points1: &[PointWithMass],
    points2: &[PointWithMass],
    dx: i64,
    dy: i64,
    res: &mut [f64],
) {
    update_moving(points1, points2, dx, dy, res);
    update_moving(points2, points1, dx, dy, res);
}

fn solve_smart(starts: &[Point], ends: &[Point], mass: &[usize]) -> Vec<f64> {
    let n = starts.len();
    let mut res = vec![f64::MAX; n];

    let mut by_shift = FxHashMap::<(i64, i64), Vec<PointWithMass>>::default();
    for i in 0..n {
        let dx = ends[i].x - starts[i].x;
        let dy = ends[i].y - starts[i].y;
        let key = (dx, dy);
        by_shift.entry(key).or_default().push(PointWithMass {
            point: starts[i],
            mass: mass[i],
            id: i,
        });
    }
    let groups = by_shift.into_iter().collect::<Vec<_>>();
    for (_, points) in groups.iter() {
        solve_stationary(points.clone(), &mut res);
    }
    for i in 0..groups.len() {
        let ((dx1, dy1), points1) = &groups[i];
        for ((dx2, dy2), points2) in groups.iter().skip(i + 1) {
            // points1 stays, points2 moves by (dx2 - dx1, dy2 - dy1)
            solve_moving(points1, points2, dx2 - dx1, dy2 - dy1, &mut res);
        }
    }

    res
}

fn solve_stupid(starts: &[Point], ends: &[Point], mass: &[usize]) -> Vec<f64> {
    let n = starts.len();
    let mut res = vec![f64::MAX; n];
    for i in 0..n {
        for j in i + 1..n {
            // i stays, j moves by (dx, dy)
            let shift_x = starts[j].x - starts[i].x;
            let shift_y = starts[j].y - starts[i].y;

            let dx = ends[j].x - starts[j].x - (ends[i].x - starts[i].x);
            let dy = ends[j].y - starts[j].y - (ends[i].y - starts[i].y);
            let min_dist2 = if dx == 0 && dy == 0 {
                (shift_x * shift_x + shift_y * shift_y) as f64
            } else {
                let l = Line::new(
                    &PointD::new(shift_x, shift_y),
                    &PointD::new(shift_x + dx, shift_y + dy),
                );
                l.abs_dist2(&PointD::ZERO).0
            };
            let dist_with_mass = min_dist2 * (mass[i] as f64) * (mass[j] as f64);
            if dist_with_mass < res[i] {
                res[i] = dist_with_mass;
            }
            if dist_with_mass < res[j] {
                res[j] = dist_with_mass;
            }
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mass = input.vec::<usize>(n);
        let mut starts = vec![];
        let mut ends = vec![];

        for _ in 0..n {
            let p1 = Point::new(input.i64(), input.i64());
            let p2 = Point::new(input.i64(), input.i64());
            starts.push(p1);
            ends.push(p2);
        }

        let res = solve_smart(&starts, &ends, &mass);
        out.println(res);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..1000);
        let mut starts = vec![];
        let mut ends = vec![];
        let mut mass = vec![];
        const MAX_X: i64 = 100;
        for _ in 0..n {
            let x = rnd.gen_range(-MAX_X..MAX_X);
            let y = rnd.gen_range(-MAX_X..MAX_X);
            let p = Point::new(x, y);
            starts.push(p);
            ends.push(p.shift(rnd.gen_range(-5..6), rnd.gen_range(-5..6)));
            mass.push(rnd.gen_range(1..100));
        }
        let res_smart = solve_smart(&starts, &ends, &mass);
        let res_stupid = solve_stupid(&starts, &ends, &mass);
        for i in 0..res_stupid.len() {
            if (res_stupid[i] - res_smart[i]).abs() > 1e-6 {
                dbg!(starts.clone(), ends.clone(), mass.clone());
                dbg!(res_stupid.clone(), res_smart.clone());
                panic!();
            }
        }
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
    let output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
