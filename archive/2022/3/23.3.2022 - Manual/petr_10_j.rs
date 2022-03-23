//{"name":"petr_10_j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"three-squares.in","pattern":null},"output":{"type":"file","fileName":"three-squares.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_j"}}}

use std::f64::consts::PI;
use std::time::Instant;

use algo_lib::f;
use algo_lib::geometry::convex_polygon_intersection::convex_polygon_intersection;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::polygon::PolygonT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<OrdF64>;
type Polygon = PolygonT<OrdF64>;

fn solve(input: &mut Input, _test_case: usize) {
    let centers: Vec<Point> = gen_vec(3, |_| input.read());
    let mut rnd = Random::new(787788);
    let start = Instant::now();
    const EPS: f64 = 2e-5;
    const FIRST_DIR: Point = Point {
        x: f!(5.0 + EPS),
        y: f!(5.0 + EPS),
    };
    while start.elapsed().as_millis() < 2000 {
        let angles = gen_vec(centers.len(), |_| rnd.gen_double() * PI);
        let rotated: Vec<_> = centers
            .iter()
            .enumerate()
            .map(|(sq_id, &center)| -> Polygon {
                let pts = gen_vec(4, |id| {
                    let angle = f!(angles[sq_id] + PI / 2.0 * id as f64);
                    FIRST_DIR.rotate_ccw_angle(angle) + center
                });
                Polygon::new(pts)
            })
            .collect();
        let mut ok = true;
        for i in 0..rotated.len() {
            for j in i + 1..rotated.len() {
                if convex_polygon_intersection(&rotated[i], &rotated[j]).is_some() {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            out_line!("PEACE");
            for a in angles.iter() {
                out_line!(*a);
            }
            return;
        }
    }
    out_line!("WAR");
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::File("three-squares.in".to_string()),
        output: TaskIoType::File("three-squares.out".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
