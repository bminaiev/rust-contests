//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};
// use rand_distr::{Distribution, Normal};

struct Query {
    plus: Vec<bool>,
    exp_sum: f64,
    res: f64,
}

fn ask(plus: &[usize], minus: &[usize], input: &mut Input) -> f64 {
    out_line!("?", plus.len(), minus.len());
    for &i in plus.iter() {
        out!(i + 1, "");
    }
    out_line!();
    for &i in minus.iter() {
        out!(i + 1, "");
    }
    out_line!();
    output().flush();
    let res = input.f64();
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let _std = input.f64();
    let mut rnd = Random::new(787788);
    let mut exp = vec![0.0; n];
    for i in 0..n {
        exp[i] = ask(&[i], &[], input);
    }

    let mut queries = vec![];
    for it in 0..n * 3 {
        let mut plus = vec![false; n];
        for i in 0..n / 2 {
            loop {
                let pos = rnd.gen(0..n);
                if !plus[pos] {
                    plus[pos] = true;
                    break;
                }
            }
        }
        let mut ask_plus = vec![];
        let mut ask_minus = vec![];
        for v in 0..n {
            if plus[v] {
                ask_plus.push(v);
            } else {
                ask_minus.push(v);
            }
        }
        let mut exp_sum = 0.0;
        for i in 0..n {
            if plus[i] {
                exp_sum += exp[i];
            } else {
                exp_sum -= exp[i];
            }
        }
        let sum = ask(&ask_plus, &ask_minus, input);
        queries.push(Query {
            plus,
            res: sum,
            exp_sum,
        });
    }
    for it in 0..50000 {
        let mut sum_sq = 0.0;
        for q in queries.iter() {
            sum_sq += (q.exp_sum - q.res).powi(2);
        }
        let v = rnd.gen(0..n);
        let delta = rnd.gen_double(); //normal.sample(&mut rng);
        exp[v] += delta;
        let mut new_sum_sq = 0.0;
        for q in queries.iter_mut() {
            if q.plus[v] {
                q.exp_sum += delta;
            } else {
                q.exp_sum -= delta;
            }
            new_sum_sq += (q.exp_sum - q.res).powi(2);
        }
        if new_sum_sq < sum_sq {
            // dbg!("woW!");
        } else {
            exp[v] -= delta;
            for q in queries.iter_mut() {
                if q.plus[v] {
                    q.exp_sum -= delta;
                } else {
                    q.exp_sum += delta;
                }
            }
        }
    }
    out!("!");
    for &x in exp.iter() {
        out!("", x);
    }
    out_line!();
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
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
