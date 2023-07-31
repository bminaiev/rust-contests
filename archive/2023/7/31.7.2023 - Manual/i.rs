//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::frac::FracT;
use algo_lib::math::gauss::gauss;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::num_traits::HasConstants;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Frac = FracT<i128>;

struct Point {
    x: Frac,
    y: Frac,
    z: Frac,
}

fn find_r(pts: &[Point]) -> Frac {
    let mut pt_rows = vec![];
    let two = Frac::new(2, 1);
    for p in pts.iter() {
        pt_rows.push(vec![
            two * p.x,
            two * p.y,
            two * p.z,
            p.x * p.x + p.y * p.y + p.z * p.z,
        ]);
    }
    let mut rows: Vec<Vec<Frac>> = vec![];
    for i in 1..pt_rows.len() {
        let mut cur_row = vec![];
        for j in 0..pt_rows[i].len() {
            cur_row.push(pt_rows[i][j] - pt_rows[0][j]);
        }
        rows.push(cur_row);
    }
    let mut a = Array2D::new_f(rows.len(), rows[0].len(), |i, j| rows[i][j]);

    let independent = gauss(&mut a);
    assert_eq!(independent, 3);
    for i in 0..3 {
        assert_eq!(a[i][i], Frac::new(1, 1));
    }
    for i in (0..3).rev() {
        for j in 0..i {
            let mul = a[j][i];
            for k in 0..a[i].len() {
                let sub = mul * a[i][k];
                a[j][k] -= sub;
            }
        }
    }
    let mut r2 = vec![];
    for p in pts.iter() {
        let mut sum = Frac::ZERO;
        for i in 0..3 {
            let coord = [p.x, p.y, p.z][i];
            let delta = a[i][3] - coord;
            sum += delta * delta;
        }
        r2.push(sum);
    }
    for i in 1..r2.len() {
        assert_eq!(r2[i], r2[0]);
    }
    r2[0]
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut rads = vec![];
    for _ in 0..n {
        let pts = gen_vec(4, |_| Point {
            x: Frac::new(input.i128(), 1),
            y: Frac::new(input.i128(), 1),
            z: Frac::new(input.i128(), 1),
        });
        rads.push(find_r(&pts));
    }
    let mut res = 0;
    for i in 0..rads.len() {
        let mut cur = 0;
        for j in 0..rads.len() {
            if rads[i] == rads[j] {
                cur += 1;
            }
        }
        res.update_max(cur);
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
