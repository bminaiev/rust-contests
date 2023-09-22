//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};
use rand_distr::{Distribution, Normal};

struct Query {
    plus: Vec<bool>,
    exp_sum: f64,
    res: f64,
}

impl Query {
    fn calc(&self, std: f64) -> f64 {
        ((self.exp_sum - self.res) / std).powi(2)
    }
}

fn stress() {
    for glob_iter in 2.. {
        let mut rr = Random::new(glob_iter + 787788);
        rr.gen_u64();
        dbg!(glob_iter);
        let std = 1.0 + rr.gen_double() * 9.0;
        let normal = Normal::new(0.0, std).unwrap();
        let mut rng = rand::thread_rng();
        let n = rr.gen(1..100);
        dbg!(n, std);
        let max_err = 7.0 * std / ((n as f64).sqrt());
        let mut rnd = Random::new(787788);
        let mut a = gen_vec(n, |_| rr.gen_double() * 900.0 + 100.0);
        let mut exp = vec![0.0; n];
        let mut exp_cnt = vec![0.0; n];

        let mut queries = vec![];
        for i in 0..n {
            exp[i] = a[i] + normal.sample(&mut rng);
            exp_cnt[i] = 1.0;
        }

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
            let mut sum = normal.sample(&mut rng);
            let mut cnt = 0.0;
            let mut exp_sum = 0.0;
            for i in 0..n {
                if plus[i] {
                    sum += a[i];
                    exp_sum += exp[i];
                    cnt += 1.0;
                } else {
                    sum -= a[i];
                    exp_sum -= exp[i];
                    cnt += 1.0;
                }
            }
            queries.push(Query {
                plus,
                res: sum,
                exp_sum,
            });
        }
        let mut sum_err = 0.0;
        for i in 0..n {
            // exp[i];
            let err = (exp[i] - a[i]).abs();
            sum_err += err;
            if err > max_err {
                dbg!(i, exp[i], a[i], err);
            }
        }
        dbg!(max_err, sum_err / n as f64);
        {
            let mut best_possible = 0.0;
            for q in queries.iter_mut() {
                let mut exp_sum = 0.0;
                for v in 0..n {
                    if q.plus[v] {
                        exp_sum += a[v];
                    } else {
                        exp_sum -= a[v];
                    }
                }
                let old_exp_sum = q.exp_sum;
                q.exp_sum = exp_sum;
                best_possible += q.calc(std);
                q.exp_sum = old_exp_sum;
            }
            dbg!(best_possible);
        }
        let mut score = 0.0;
        for it in 0..50000 {
            let mut sum_sq = 0.0;
            for q in queries.iter() {
                sum_sq += q.calc(std);
            }
            // dbg!(sum_sq);
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
                new_sum_sq += q.calc(std);
            }
            if new_sum_sq < sum_sq {
                score = new_sum_sq;
                // dbg!("woW!", sum_sq);
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
        dbg!(score);
        let mut sum_err = 0.0;
        for i in 0..n {
            // exp[i];
            let err = (exp[i] - a[i]).abs();
            sum_err += err;
            if err > max_err {
                dbg!(i, exp[i], a[i], err);
                unreachable!();
            }
        }
        dbg!(max_err, sum_err / n as f64);
    }
}

fn solve(input: &mut Input, _test_case: usize) {}

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
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
