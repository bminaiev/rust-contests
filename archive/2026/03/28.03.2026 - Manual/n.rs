//{"name":"n","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::big_int::BigInt;
use algo_lib::misc::rand::Random;

fn check_ok(mask1: usize, mask2: usize, a: &[usize]) -> bool {
    let n = a.len();
    let mut alls = vec![BigInt::new(100_000)];
    let mut sign = vec![0];
    for i in 0..n {
        if (mask1 >> i) & 1 == 1 {
            alls.push(BigInt::new(a[i] as i64));
            sign.push(1);
        }
        if (mask2 >> i) & 1 == 1 {
            alls.push(BigInt::new(a[i] as i64));
            sign.push(-1);
        }
    }
    let mut sum_pos = BigInt::new(0);
    let mut sum_neg = BigInt::new(0);
    let mut sum_delta = BigInt::new(0);
    for i in 0..alls.len() {
        let mut others = BigInt::new(1);
        for j in 0..alls.len() {
            if i != j {
                others = others * alls[j].clone();
            }
        }
        if sign[i] == 1 {
            sum_pos = sum_pos + others;
        } else if sign[i] == -1 {
            sum_neg = sum_neg + others;
        } else {
            sum_delta = sum_delta + others;
        }
    }
    if (sum_pos.clone() + sum_delta.clone()) < sum_neg.clone() {
        return false;
    }
    if (sum_neg.clone() + sum_delta.clone()) < sum_pos.clone() {
        return false;
    }
    true
}

fn solve_case(a: &[usize]) -> Option<(usize, usize)> {
    let start = Instant::now();
    let n = a.len();
    let a_inv: Vec<f64> = a.iter().cloned().map(|x| 1.0 / (x as f64)).collect();
    let mut by_mask = vec![vec![]; n + 1];
    for mask in 1usize..(1 << n) {
        let mut sum = 0.0;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                sum += a_inv[i];
            }
        }
        by_mask[mask.count_ones() as usize].push((sum, mask));
    }
    for v in by_mask.iter_mut() {
        v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }
    const EPS_HOPE: f64 = 1e-5 + 1e-6;

    for eps in [EPS_HOPE] {
        for cnt in 1..=n {
            for w in by_mask[cnt].windows(2) {
                if start.elapsed().as_secs_f64() > 1.8 {
                    return None;
                }

                if (w[1].0 - w[0].0).abs() <= eps {
                    dbg!((w[1].0 - w[0].0).abs(), w[0].1, w[1].1);
                    if check_ok(w[0].1, w[1].1, a) {
                        return Some((w[0].1, w[1].1));
                    }
                }
            }
        }
    }
    None
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize().min(22);
    let a = input.vec::<usize>(n);

    for i in 0..n {
        for j in i + 1..n {
            if a[i] == a[j] {
                out.println("Yes");
                out.println(1);
                out.println(vec![i + 1]);
                out.println(vec![j + 1]);
                return;
            }
        }
    }

    if let Some((mask1, mask2)) = solve_case(&a) {
        out.println("Yes");
        let cnt = mask1.count_ones();
        out.println(cnt);
        for mask in [mask1, mask2] {
            let mut ans = vec![];
            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    ans.push(i + 1);
                }
            }
            out.println(ans);
        }
    } else {
        out.println("No");
    }
}

fn stress() {
    let n = 22;
    let mut a = vec![];
    let mut rnd = Random::new(2314234);
    for i in 0..n {
        a.push(rnd.next_in_range(1, 100_000));
    }
    let r = solve_case(&a);
    dbg!(r);
    for _ in 0..1000 {
        let mask1 = rnd.gen_range(1..1 << n);
        let mask2 = rnd.gen_range(1..1 << n);
        let ok = check_ok(mask1, mask2, &a);
        dbg!(mask1, mask2, ok);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "n";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    run_single_test(PROBLEM_NAME, run, "4");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
