use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::SearchFor::MinimumScore;
use algo_lib::misc::simulated_annealing::SimulatedAnnealing;

fn calc(a: &[usize]) -> usize {
    let mut res = 0;
    for &x in a.iter() {
        if (res ^ x) > res {
            res ^= x;
        }
    }
    res
}

fn stress() {
    let k = 10;
    let n = k * 20;
    let mut rnd = Random::new(13);
    let mut a = rnd.gen_vec(n, 0..1 << k);
    let mut sa = SimulatedAnnealing::new(10.0, MinimumScore, 10.0, 1e-3, calc(&a));
    let mut iter = 0;
    while sa.should_continue() {
        iter += 1;
        if iter % 1000 == 0 {
            let res = calc(&a);
            let res_binary = format!("{:0width$b}", res, width = k);
            dbg!(iter, res, res_binary);
        }
        let mut na = a.clone();
        let idx = rnd.gen_range(0..n);
        let value = a[idx];
        na.remove(idx);
        let nidx = rnd.gen_range(0..n);
        na.insert(nidx, value);
        // let res = calc(&a);
        let nres = calc(&na);
        if sa.should_go(nres) {
            a = na;
        }
    }
}

fn stress123() {
    let k = 5;
    let n = 10;
    // how many rounds more, current state
    let mut dp = Array2D::new(0.0, n + 1, 1 << k);
    for mask in 0..1 << k {
        dp[0][mask] = mask as f64;
    }
    const EPS: f64 = 1e-9;
    for round in 1..=n {
        for cur_mask in 0..(1 << k) {
            let mut res = 0.0;
            for num in 0..(1 << k) {
                let nmask = cur_mask ^ num;
                let want_to_take = nmask >= cur_mask;
                let tmp_res = if dp[round - 1][nmask] < dp[round - 1][cur_mask] - EPS {
                    // do not take
                    if want_to_take {
                        dbg!(
                            "want to take, but shouldn't",
                            cur_mask,
                            nmask,
                            dp[round - 1][cur_mask],
                            dp[round - 1][nmask]
                        );
                    }
                    dp[round - 1][cur_mask]
                } else {
                    if !want_to_take {
                        let delta = dp[round - 1][nmask] - dp[round - 1][cur_mask];
                        if delta.abs() > EPS {
                            dbg!(
                                "don't want, but should!",
                                cur_mask,
                                nmask,
                                dp[round - 1][cur_mask],
                                dp[round - 1][nmask]
                            );
                        }
                    }
                    dp[round - 1][nmask]
                };
                res += tmp_res;
            }
            res /= (1 << k) as f64;
            dp[round][cur_mask] = res;
        }
        dbg!("round", round);
        for mask in 0..1 << k {
            dbg!(mask, dp[round][mask]);
            if mask > 0 {
                assert!(dp[round][mask - 1] <= dp[round][mask] + EPS);
            }
        }
    }
    dbg!(dp[n][0]);
}

fn solve(input: &mut Input, out: &mut Output) {}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "g";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
