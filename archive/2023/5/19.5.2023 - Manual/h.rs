//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use std::cmp::max;

use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::seg_trees::fenwick_max::FenwickMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.i32();
    let a = gen_vec(n, |_| Point::new(input.i64(), input.i64()));

    let mut ids: Vec<_> = (0..n).collect();

    let mut fenw = FenwickMax::<i32>::new(n);

    let mut left = -1e10;
    let mut right = 1e10;
    for _ in 0..100 {
        let check_angle = (left + right) / 2.0;
        // check_angle = -1.0;
        ids.sort_by(|&i1, &i2| {
            let p1 = a[i1];
            let f1 = p1.y as f64 - check_angle * p1.x as f64;
            let p2 = a[i2];
            let f2 = p2.y as f64 - check_angle * p2.x as f64;
            f1.partial_cmp(&f2).unwrap()
        });
        // dbg!(ids);
        fenw.clear();
        let mut max_ans = 0;
        for &id in ids.iter() {
            let pref = max(0, fenw.get_range_max(id));
            max_ans.update_max(pref + 1);
            fenw.add(id, pref + 1);
        }
        if max_ans >= k {
            left = check_angle;
        } else {
            right = check_angle;
        }
    }
    let res = (left + right) / 2.0;
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
