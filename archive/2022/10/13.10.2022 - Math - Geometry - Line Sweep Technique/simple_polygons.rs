//{"name":"Simple Polygons","group":"HackerEarth - Math - Geometry - Line Sweep Technique","url":"https://www.hackerearth.com/practice/math/geometry/line-sweep-technique/practice-problems/algorithm/simple-polygon-eadaf4dd/?","interactive":false,"timeLimit":4000,"tests":[{"input":"1\n4\n9\n1 1\n7 0\n10 1\n12 6\n12 0\n14 7\n7 8\n2 7\n1 5\n5\n3 3\n4 4\n3 5\n4 6\n2 6\n7\n6 1\n10 3\n8 4\n10 6\n7 7\n6 5\n5 4\n6\n13 1\n15 1\n16 2\n17 1\n17 4\n15 5\n20\n1 3 4\n2 2\n1 5 7\n2 2\n2 1\n1 7 3\n2 1\n2 2\n2 3\n1 11 3\n1 10 8\n1 13 4\n2 1\n2 2\n2 3\n1 14 2\n2 1\n2 2\n2 3\n2 4\n","output":"1\n1\n2\n3\n1\n1\n4\n1\n1\n4\n1\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SimplePolygons"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::geometry::point_location::PointLocation;
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
    let polygons = gen_vec(n, |_| {
        gen_vec(input.usize(), |_| Point::new(input.read(), input.read()))
    });
    let point_location = PointLocation::new(&polygons);
    let queries = input.usize();
    let mut inside = vec![0; n];
    for _ in 0..queries {
        let q_type = input.usize();
        if q_type == 1 {
            let x = input.i64();
            let y = input.i64();
            if let Some(mut p) = point_location.locate_point(Point::new(x, y), true) {
                loop {
                    inside[p] += 1;
                    if let Some(np) = point_location.parents[p] {
                        p = np;
                    } else {
                        break;
                    }
                }
            }
        } else {
            assert!(q_type == 2);
            out_line!(inside[input.usize() - 1]);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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
    tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
