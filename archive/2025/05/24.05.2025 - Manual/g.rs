//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve_fast(ls: &[i64], k: i64) -> f64 {
    let n = ls.len();
    let mut left = 1e-9;
    let mut right = 100.0;
    for _ in 0..100 {
        let eps = (left + right) / 2.0;
        let mut can_fit = true;

        let mut need_t = 0;
        for i in 0..n {
            let t_more = 1.0 / eps - 100.0 / (ls[i] as f64);
            let real_t = (t_more.ceil() as i64).max(0);
            need_t += real_t;
            if need_t > k {
                can_fit = false;
                break;
            }
        }
        if can_fit {
            right = eps;
        } else {
            left = eps;
        }
    }

    // dbg!(left);

    {
        let eps = right;

        let mut can_fit = true;

        let mut need_t = 0;
        let mut res = 1.0;
        for i in 0..n {
            let t_more = 1.0 / eps - 100.0 / (ls[i] as f64);
            let real_t = (t_more.ceil() as i64).max(0);
            res *= 1.0 + (ls[i] as f64) * (real_t as f64) / 100.0;
            // dbg!(i, real_t, res);
            need_t += real_t;
            if need_t > k {
                can_fit = false;
                break;
            }
        }
        assert!(can_fit);
        let more = k - need_t;
        // dbg!(more);
        res *= (1.0 + eps).powi(more as i32);
        res
    }
}

fn solve_slow(ls: &[i64], k: i64) -> f64 {
    let n = ls.len();
    let mut ts = vec![0; n];
    for _ in 0..k {
        let mut coefs = vec![0.0; n];
        for i in 0..n {
            let cur = 1.0 + (ls[i] as f64) * (ts[i] as f64) / 100.0;
            let next = 1.0 + (ls[i] as f64) * ((ts[i] + 1) as f64) / 100.0;
            coefs[i] = next / cur;
        }
        let mut best_i = 0;
        for i in 1..n {
            if coefs[i] > coefs[best_i] {
                best_i = i;
            }
        }
        ts[best_i] += 1;
    }
    let mut res = 1.0;
    for i in 0..n {
        res *= 1.0 + (ls[i] as f64) * (ts[i] as f64) / 100.0;
    }
    res
}

fn stress() {
    for it in 51.. {
        dbg!(it);
        let mut rng = Random::new(it);
        let n = rng.gen(1..10);
        let k = rng.gen(1..100);
        let mut ls = rng.gen_vec(n, 1..101);
        // for i in 0..n {
        //     ls[i] *= 25;
        //     assert!(ls[i] <= 100);
        // }

        let slow_ans = solve_slow(&ls, k);
        let fast_ans = solve_fast(&ls, k);
        let relative_diff = ((slow_ans - fast_ans).abs() / slow_ans).abs();
        if relative_diff > 1e-4 {
            dbg!(ls);
            dbg!(k);
            dbg!(slow_ans);
            dbg!(fast_ans);
            dbg!(relative_diff);
            panic!("WA");
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.i64();
        let ls = input.vec::<i64>(n);
        out.println(solve_fast(&ls, k));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "g";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
