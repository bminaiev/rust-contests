//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::exp_iter::ExpIter;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let w = Array2D::new_f(n, n, |_, _| input.f64());
    let mut best_score = 0.0;
    let mut rnd = Random::new(787788);
    for _ in 0..1000 {
        let mut a = gen_vec(n, |_| rnd.gen_double());
        fix(&mut a);
        let mut score = calc_score(&w, &a);
        for t in ExpIter::new(1.0..1e-9, 0.5 / 1000.0).into_iter() {
            let i = rnd.gen(0..n);
            let mut new_a = a.clone();
            if rnd.gen_bool() {
                new_a[i] += rnd.gen_double() * t;
            } else {
                new_a[i] -= rnd.gen_double() * t;
            }
            if new_a[i] < 0.0 {
                new_a[i] = 0.0;
            }
            fix(&mut new_a);
            let new_score = calc_score(&w, &new_a);
            if new_score > score {
                score = new_score;
                a = new_a;
            }
        }
        if score > best_score {
            best_score = score;
        }
    }
    out_line!(best_score);
}

fn calc_score(w: &Array2D<f64>, a: &[f64]) -> f64 {
    let n = w.len();
    let mut res = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            res += w[i][j] * a[i] * a[j];
        }
    }
    res
}

fn fix(a: &mut [f64]) {
    let sum = a.iter().sum::<f64>();
    for x in a.iter_mut() {
        *x /= sum;
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
