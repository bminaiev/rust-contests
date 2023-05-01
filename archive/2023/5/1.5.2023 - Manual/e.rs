//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use std::f64::consts::PI;

use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

struct Circle {
    center: Point,
    r: i64,
}

#[derive(Clone, Copy)]
struct Segment {
    mid_angle: f64,
    delta: f64,
}

fn exist_list(a: &[Circle]) -> bool {
    for c in a.iter() {
        if c.center.dist2(&Point::ZERO) <= c.r * c.r {
            return false;
        }
    }
    let mut segs = vec![];
    for c in a.iter() {
        let mut mid_angle = f64::atan2(c.center.y as f64, c.center.x as f64);
        if mid_angle < 0.0 {
            mid_angle += 2.0 * std::f64::consts::PI;
        }
        let delta = f64::asin(c.r as f64 / (c.center.dist2(&Point::ZERO) as f64).sqrt());
        segs.push(Segment { mid_angle, delta })
    }
    segs.sort_by(|a, b| a.mid_angle.partial_cmp(&b.mid_angle).unwrap());

    let mut min_x = std::f64::MAX;
    let mut max_x = -std::f64::MAX;
    for seg in segs.iter() {
        {
            let x = seg.mid_angle + seg.delta;
            if x > max_x {
                max_x = x;
            }
        }
        {
            let x = seg.mid_angle - seg.delta;
            if x < min_x {
                min_x = x;
            }
        }
    }
    if max_x - min_x < std::f64::consts::PI {
        return true;
    }
    {
        let mut max_x = 0.0;
        let mut min_x = 0.0;
        for s in segs.iter() {
            if s.mid_angle - s.delta < PI && s.mid_angle + s.delta > PI {
                return false;
            }
            if s.mid_angle < PI {
                let x = s.mid_angle + s.delta;
                if x > max_x {
                    max_x = x;
                }
            } else {
                let x = s.mid_angle - s.delta - PI * 2.0;
                if x < min_x {
                    min_x = x;
                }
            }
        }
        max_x - min_x < PI
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| Circle {
        center: Point::new(input.read(), input.read()),
        r: input.read(),
    });
    if !exist_list(&a) {
        out_line!("Yes");
    } else {
        out_line!("No");
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
