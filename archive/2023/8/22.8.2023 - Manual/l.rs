//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

// shoudl be >= 150

fn calc_range(start: i32, end: i32) -> (f64, f64) {
    const X: i32 = 150;
    if start >= X && end >= X {
        return (0.0, 1.0);
    }
    if start < X && end < X {
        return (0.0, 0.0);
    }
    if start >= X {
        let left_sz = start - X;
        let right_sz = X - end;
        let mid = left_sz as f64 / (left_sz + right_sz) as f64;
        return (0.0, mid);
    }
    let left_sz = X - start;
    let right_sz = end - X;
    let mid = left_sz as f64 / (left_sz + right_sz) as f64;
    (mid, 1.0)
}

fn intersect(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (f64::max(a.0, b.0), f64::min(a.1, b.1))
}

fn calc_size(l1: i64, l2: i64, pos: f64) -> f64 {
    let l1 = l1 as f64;
    let l2 = l2 as f64;
    l1 * (1.0 - pos) + l2 * pos
}

fn solve_case(
    x1: i64,
    y1: i64,
    y2: i64,
    left: &[i32],
    x2: i64,
    y3: i64,
    y4: i64,
    right: &[i32],
) -> f64 {
    let mut ok = (0.0, 1.0);
    ok = intersect(ok, calc_range(left[0], right[0]));
    let inv = |color: i32| 150 + (75 - color);
    ok = intersect(ok, calc_range(inv(left[1]), inv(right[1])));
    ok = intersect(ok, calc_range(inv(left[2]), inv(right[2])));
    if ok.0 > ok.1 {
        return 0.0;
    }
    let sz1 = calc_size(y2 - y1, y4 - y3, ok.0);
    let sz2 = calc_size(y2 - y1, y4 - y3, ok.1);
    let mid = (sz1 + sz2) / 2.0;
    let dist = (x2 - x1) as f64 * (ok.1 - ok.0);
    dist * mid
}

fn solve(input: &mut Input, _test_case: usize) {
    let x1 = input.i64();
    let y1 = input.i64();
    let y2 = input.i64();
    let mut left = input.vec::<i32>(3);

    let x2 = input.i64();
    let y3 = input.i64();
    let y4 = input.i64();
    let mut right = input.vec::<i32>(3);

    for _it in 0..3 {
        let res = solve_case(x1, y1, y2, &left, x2, y3, y4, &right);
        left.rotate_left(1);
        right.rotate_left(1);
        out_line!(res);
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
