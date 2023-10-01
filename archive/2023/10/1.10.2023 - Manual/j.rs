//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| Point {
        x: input.f64(),
        y: input.f64(),
        z: input.f64(),
    });
    let mut exist_good = false;
    let mut res = std::f64::MAX;
    for i in 0..n {
        for j in 0..n {
            let dir1 = Point {
                x: a[j].x - a[i].x,
                y: a[j].y - a[i].y,
                z: a[j].z - a[i].z,
            };
            for k in 0..n {
                for l in 0..n {
                    let dir2 = Point {
                        x: a[l].x - a[k].x,
                        y: a[l].y - a[k].y,
                        z: a[l].z - a[k].z,
                    };
                    let mut vec_mul = Point {
                        x: dir1.y * dir2.z - dir1.z * dir2.y,
                        y: dir1.z * dir2.x - dir1.x * dir2.z,
                        z: dir1.x * dir2.y - dir1.y * dir2.x,
                    };
                    let sz =
                        (vec_mul.x * vec_mul.x + vec_mul.y * vec_mul.y + vec_mul.z * vec_mul.z)
                            .sqrt();
                    if sz > 1e-5 {
                        exist_good = true;
                        vec_mul.x /= sz;
                        vec_mul.y /= sz;
                        vec_mul.z /= sz;

                        let mut min1 = std::f64::MAX;
                        let mut max1 = std::f64::MIN;
                        for p in 0..n {
                            let cur = a[p].x * vec_mul.x + a[p].y * vec_mul.y + a[p].z * vec_mul.z;
                            min1 = min1.min(cur);
                            max1 = max1.max(cur);
                        }
                        let delta = max1 - min1;
                        res = res.min(delta);
                    }
                }
            }
        }
    }
    if !exist_good {
        res = 0.0;
    }
    out_line!(res);
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
