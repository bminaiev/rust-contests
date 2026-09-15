use std::assert_eq;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::math::primes::{factorize, gen_largest_prime_table};
use algo_lib::misc::binary_search::{binary_search_first_true, binary_search_last_true};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);

        let largest_primes = gen_largest_prime_table(n + 1);

        let mut g: Vec<Vec<usize>> = vec![vec![]; n + 1];
        for base in 1..=n {
            let mut only_primes = true;
            for prime in factorize(&largest_primes, base) {
                if prime.power != 1 {
                    only_primes = false;
                    break;
                }
            }
            if !only_primes {
                continue;
            }
            let mut jumps = vec![];
            for mult in (1..=n / base).rev() {
                let mut other_primes = false;
                for prime in factorize(&largest_primes, mult) {
                    if base % prime.value != 0 {
                        other_primes = true;
                        break;
                    }
                }
                g[base * mult].extend(jumps.iter().cloned());
                if !other_primes {
                    jumps.push(mult * base);
                }
            }
        }
        for x in 1..=n {
            g[x].sort();
        }

        let mut dp2: Vec<Vec<Answer>> = vec![vec![]; n + 1];
        for min in 1..=n {
            dp2[min].push(Answer {
                answer: min,
                r: min,
            });
            for &to in g[min].iter() {
                let idx =
                    binary_search_last_true(0..dp2[min - 1].len(), |i| dp2[min - 1][i].r < to)
                        .unwrap();
                let answer = dp2[min - 1][idx].answer;
                dp2[min].push(Answer { answer, r: to });
            }
        }
        let mut cnt = vec![0; n + 1];
        for x in a.iter() {
            cnt[*x] += 1;
        }
        let mut pref_cnt = vec![0; n + 2];
        for i in 1..=n {
            pref_cnt[i + 1] = pref_cnt[i] + cnt[i];
        }
        let pow2 = Mod::gen_powers(Mod::new(2), n + 1);
        let mut res = Mod::ZERO;
        for min_value in 1..=n {
            if cnt[min_value] == 0 {
                continue;
            }
            for i in 0..dp2[min_value].len() {
                let from_end = dp2[min_value][i].r;
                let to_end = if i + 1 < dp2[min_value].len() {
                    dp2[min_value][i + 1].r
                } else {
                    n + 1
                };
                let total_cnt = pref_cnt[to_end] - pref_cnt[min_value];
                let cnt_right = pref_cnt[to_end] - pref_cnt[from_end];
                let cnt_left = cnt[min_value];
                let ways = if from_end == min_value {
                    let cnt_right = cnt_right - cnt_left;
                    (pow2[cnt_left] - Mod::ONE) * pow2[cnt_right]
                } else {
                    pow2[total_cnt] - pow2[total_cnt - cnt_left] - pow2[total_cnt - cnt_right]
                        + pow2[total_cnt - cnt_left - cnt_right]
                };
                res += ways * Mod::new(dp2[min_value][i].answer);
            }
        }
        out.println(res);
    }
}

fn stress() {
    let n = 300000;
    let largest_primes = gen_largest_prime_table(n + 1);

    let mut g: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for base in 1..=n {
        let mut only_primes = true;
        for prime in factorize(&largest_primes, base) {
            if prime.power != 1 {
                only_primes = false;
                break;
            }
        }
        if !only_primes {
            continue;
        }
        let mut jumps = vec![];
        for mult in (1..=n / base).rev() {
            let mut other_primes = false;
            for prime in factorize(&largest_primes, mult) {
                if base % prime.value != 0 {
                    other_primes = true;
                    break;
                }
            }
            g[base * mult].extend(jumps.iter().cloned());
            if !other_primes {
                jumps.push(mult * base);
            }
        }
    }
    let mut sum = 0;
    for x in 1..=n {
        g[x].sort();
        sum += g[x].len();
    }
    dbg!(sum);

    let mut dp2: Vec<Vec<Answer>> = vec![vec![]; n + 1];
    for min in 1..=n {
        dp2[min].push(Answer {
            answer: min,
            r: min,
        });
        for &to in g[min].iter() {
            let idx =
                binary_search_last_true(0..dp2[min - 1].len(), |i| dp2[min - 1][i].r < to).unwrap();
            let answer = dp2[min - 1][idx].answer;
            dp2[min].push(Answer { answer, r: to });
        }
    }

    // let mut dp = Array2D::new(0, n + 1, n + 1);
    // let mut cc = 0;
    // for min in 1..=n {
    //     for max in min..=n {
    //         if min == max {
    //             dp[min][max] = min;
    //         } else {
    //             let mut exist_prime = false;
    //             for prime in factorize(&largest_primes, max) {
    //                 if min % prime.value != 0 {
    //                     exist_prime = true;
    //                     break;
    //                 }
    //             }
    //             let found = g[min].binary_search(&max).is_ok();
    //             assert_eq!(found, !exist_prime);
    //             if exist_prime {
    //                 dp[min][max] = dp[min][max - 1];
    //             } else {
    //                 dp[min][max] = dp[min - 1][max - 1];
    //                 cc += 1;
    //             }
    //             {
    //                 let idx = binary_search_last_true(0..dp2[min].len(), |i| dp2[min][i].r <= max)
    //                     .unwrap();
    //                 assert!(dp[min][max] == dp2[min][idx].answer);
    //             }
    //         }
    //     }
    // }
    // dbg!(cc);
    // let mut cnt = 0;
    // for min in 1..n {
    //     for max in min + 1..=n {
    //         if dp[min][max] != dp[min][max - 1] {
    //             // dbg!(min, max, dp[min][max]);
    //             cnt += 1;
    //         }
    //     }
    // }
    // dbg!(cnt);
}

#[derive(Clone, Copy)]
struct Answer {
    answer: usize,
    r: usize,
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e1_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
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
