//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::f;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<OrdF64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize() + 1;
    let d = input.f64();
    let d2 = f!(d * d);
    let mut starts = vec![];
    let mut dirs = vec![];
    for _ in 0..n {
        starts.push(Point::new(input.read(), input.read()));
        dirs.push(Point::new(input.read(), input.read()));
    }
    const MX: f64 = std::f64::MAX / 10.0;
    let mut meet_from = Array2D::new(MX, n, n);
    let mut meet_to = Array2D::new(-1.0, n, n);
    const EPS: f64 = 1e-10;
    for i in 0..n {
        for j in i + 1..n {
            let start = starts[j] - starts[i];
            let dir = dirs[j] - dirs[i];
            if dir == Point::ZERO {
                if start.dist2(&Point::ZERO) <= d2 {
                    meet_from[i][j] = 0.0;
                    meet_to[i][j] = MX;
                    meet_from[j][i] = 0.0;
                    meet_to[j][i] = MX;
                }
                continue;
            }
            let start_to = start + dir;
            let line = Line::new(&start, &(start_to));
            let smallest_d2 = line.abs_dist2(&Point::ZERO);
            if smallest_d2 <= d2 + f!(EPS) {
                let closest = line.closest_to_zero();
                let mut dist_to_closest = closest.dist2(&start).sqrt();
                if Point::scal_mul(&start, &start_to, &closest) <= OrdF64::ZERO {
                    dist_to_closest = -dist_to_closest;
                }
                let ok_delta = (d2 - smallest_d2).sqrt();
                let speed = dir.dist2(&Point::ZERO).sqrt();
                let mut from = (dist_to_closest - ok_delta) / speed;
                let to = (dist_to_closest + ok_delta) / speed;
                if to >= OrdF64::ZERO - f!(EPS) {
                    if from < OrdF64::ZERO {
                        from = OrdF64::ZERO;
                    }
                    meet_from[i][j] = from.0;
                    meet_from[j][i] = from.0;
                    meet_to[i][j] = to.0;
                    meet_to[j][i] = to.0;
                }
            }
        }
    }
    let mut dist = vec![MX; n];
    dist[0] = 0.0;
    let mut seen = vec![false; n];
    loop {
        let mut min_dist = MX;
        let mut best_from = n;
        for v in 0..n {
            if !seen[v] && dist[v] < min_dist {
                min_dist = dist[v];
                best_from = v;
            }
        }
        if best_from == n {
            break;
        }
        seen[best_from] = true;
        let cur_time = dist[best_from];
        for to in 0..n {
            if meet_to[best_from][to] >= cur_time - EPS {
                let meet_time = if meet_from[best_from][to] < cur_time {
                    cur_time
                } else {
                    meet_from[best_from][to]
                };
                if dist[to] > meet_time {
                    dist[to] = meet_time;
                }
            }
        }
    }
    for v in 1..n {
        if dist[v] > MX / 2.0 {
            out_line!(-1);
        } else {
            out_line!(dist[v]);
        }
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
