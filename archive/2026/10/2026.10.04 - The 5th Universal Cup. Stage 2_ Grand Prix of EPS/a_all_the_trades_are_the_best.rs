use std::ops::Range;
use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

#[target_feature(enable = "avx2")]
fn solve_case(a: &[i32], queries: &[Range<usize>], k: usize) -> Vec<i32> {
    let q = queries.len();
    let mut res = vec![i32::MIN; q];
    let mut dp_left = Array2D::new(0, a.len() / 10 + 10, k + 1);
    loop {
        let mut min_r = usize::MAX;
        for i in 0..q {
            if res[i] == i32::MIN {
                min_r = min_r.min(queries[i].end);
            }
        }
        if min_r == usize::MAX {
            break;
        }
        let mid = min_r;
        let mut max_r = mid;
        let mut min_l = usize::MAX;
        for i in 0..q {
            if res[i] == i32::MIN && queries[i].start < mid {
                max_r = max_r.max(queries[i].end);
                min_l = min_l.min(queries[i].start);
            }
        }

        let mut by_right = vec![];
        for i in 0..q {
            if res[i] != i32::MIN || queries[i].start >= mid {
                continue;
            }
            let right_len = queries[i].end - mid;
            by_right.push((right_len, i, queries[i].start));
        }
        by_right.sort_unstable();

        for use_center in 0..2 {
            {
                // dp[len][segm_open?][cnt_used]
                let len = mid - min_l;

                let mut dp = Array2D::new(i32::MIN / 2, 2, k + 1);
                let mut ndp = Array2D::new(i32::MIN / 2, 2, k + 1);

                dp[use_center][0] = 0;
                for i in 0..len {
                    let cur_val = a[mid - 1 - i];
                    // end of segment
                    update_dp(&mut ndp[1][1..], &dp[1][1..], &dp[0][..k], cur_val);
                    ndp[1][0] = dp[1][0];
                    // start new segment
                    update_dp(&mut ndp[0], &dp[0], &dp[1], -cur_val);
                    dp_left[i].copy_from_slice(&dp[0]);
                    std::mem::swap(&mut dp, &mut ndp);
                }
                dp_left[len].copy_from_slice(&dp[0]);
            };

            {
                // dp[len][segm_open?][cnt_used]
                let len = max_r - mid;
                let mut dp = Array2D::new(i32::MIN / 2, 2, k + 1);
                let mut ndp = Array2D::new(i32::MIN / 2, 2, k + 1);
                dp[use_center].fill(0);
                let mut q_it = 0;
                for i in 0..=len {
                    while q_it < by_right.len() && by_right[q_it].0 == i {
                        let (right_len, qid, qstart) = by_right[q_it];
                        assert_eq!(right_len, i);
                        q_it += 1;
                        let left_len = mid - qstart;

                        let best = fast_conv(&dp_left[left_len], &dp[0]);

                        res[qid] = res[qid].max(best);
                    }
                    if i == len {
                        break;
                    }

                    let cur_val = a[mid + i];
                    // open new segment
                    update_dp(&mut ndp[1], &dp[1], &dp[0], -cur_val);
                    // close segment
                    update_dp(&mut ndp[0][1..], &dp[0][1..], &dp[1][..k], cur_val);

                    ndp[0][0] = dp[0][0];
                    std::mem::swap(&mut dp, &mut ndp);
                }
            };
        }
    }
    res
}

// ndp[i] = max(dp0[i], dp1[i] + cur_val)
fn update_dp(ndp: &mut [i32], dp0: &[i32], dp1: &[i32], cur_val: i32) {
    let k = dp0.len();
    assert_eq!(ndp.len(), k);
    assert_eq!(dp1.len(), k);
    for j in 0..k {
        ndp[j] = dp0[j].max(dp1[j] + cur_val);
    }
}

fn fast_conv(a: &[i32], b: &[i32]) -> i32 {
    let mut res = i32::MIN;
    let n = a.len();
    assert_eq!(n, b.len());
    for i in 0..n {
        res = res.max(a[i] + b[n - 1 - i]);
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let k = input.usize();
    let q = input.usize();
    let mut a = input.vec::<i32>(n);
    a.extend_from_slice(&a.clone());
    let mut queries = vec![];
    for _ in 0..q {
        let l = input.usize() - 1;
        let mut r = input.usize();
        if l >= r {
            r += n;
        }
        queries.push(l..r);
    }
    let res = unsafe { solve_case(&a, &queries, k) };
    for &x in res.iter() {
        out.println(x);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let n = 100_000;
        let q = 300_000;
        let mut rnd = Random::new(it);
        let mut a = rnd.gen_vec(n, 0..1_000_000);
        a.extend_from_slice(&a.clone());
        let mut queries = vec![];
        for _ in 0..q {
            let l = rnd.gen_range(0..n);
            let fr = (0.15 * n as f64) as usize;
            let to = (0.85 * n as f64) as usize;
            let len = rnd.gen_range(fr..to);
            let r = l + len;
            queries.push(l..r);
        }
        let start = Instant::now();
        let res = unsafe { solve_case(&a, &queries, 800) };
        let res_xor = res.iter().fold(0, |acc, &x| acc ^ x);
        dbg!(res_xor);
        dbg!(start.elapsed());
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_all_the_trades_are_the_best";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
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
