//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use std::cmp::max;

use algo_lib::geometry::point::PointT;
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
    let k = input.usize();
    let a = gen_vec(n, |_| Point::new(input.i64(), input.i64()));

    let side = |mut i: usize, mut j: usize| -> i64 {
        i %= n;
        j %= n;
        let p1 = a[i];
        let p2 = a[j];
        (p1.x * p2.y - p1.y * p2.x)
    };

    let tr = |i: usize, j: usize, k: usize| -> i64 { side(i, j) + side(j, k) + side(k, i) };

    let mut ans = 0;

    let mut cur_area = 0;
    for i in 0..k {
        cur_area += side(i, i + 1);
    }
    cur_area += side(k, 0);

    let mut it = k;

    for start in 0..n {
        loop {
            let now_tr = tr(start, start + k, it);
            let next_tr = tr(start, start + k, it + 1);
            if next_tr >= now_tr {
                it += 1;
            } else {
                break;
            }
        }
        let sum_area = cur_area + tr(start, start + k, it);
        ans = max(ans, sum_area);

        cur_area -= side(start, start + 1);
        cur_area += side(start + k, start + k + 1);
        cur_area -= side(start + k, start);
        cur_area += side(start + k + 1, start + 1);
    }

    if ans % 2 == 0 {
        out_line!(ans / 2);
    } else {
        out_line!(format!("{}.5", ans / 2));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
