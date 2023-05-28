//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use std::cmp::max;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |_| Point::new(input.i64(), input.i64()));
    for i in 0..n {
        let p = a[i].clone();
        a.push(p);
    }
    let mut dp = Array2D::new(0, a.len(), a.len());
    let mut ok = Array2D::new(false, a.len(), a.len());
    for i in (0..dp.len()).rev() {
        let mut pref = 0;
        for j in i + 1..dp.len() {
            let dist = a[i].dist2(&a[j]);
            pref.update_max(dist);
            let cur = max(dp[i + 1][j], pref);
            dp[i][j] = cur;

            if ok[i + 1][j] {
                ok[i][j] = true;
            }
            if Point::vect_mul(&a[i], &a[i + 1], &a[j]) != 0 {
                ok[i][j] = true;
            }
        }
    }
    let mut res = std::i64::MAX;
    for i in 0..n {
        for len in 2..=(n - 2) {
            let len2 = n - len;
            let d1 = dp[i][i + len];
            let d2 = dp[i + len][i + len + len2];
            let cur = d1 + d2;
            if ok[i][i + len] && ok[i + len][i + len + len2] {
                res.update_min(cur);
            }
        }
    }
    out_line!(res);
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
