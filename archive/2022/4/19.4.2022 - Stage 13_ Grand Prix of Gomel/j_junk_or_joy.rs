//{"name":"J. Junk or Joy","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/J/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n22\n","output":"3\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JJunkOrJoy"}}}

use std::collections::HashSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::primes::gen_primes_table;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn is_prime_power(n: i64, check_up_to: i64) -> Option<(i64, i64)> {
    if n == 1 {
        return None;
    }
    for prime in 2..n {
        if n % prime == 0 {
            let mut pw = 0;
            let mut cur_n = n;
            while cur_n % prime == 0 {
                cur_n /= prime;
                pw += 1;
            }
            if cur_n == 1 {
                return Some((prime, pw));
            } else {
                return None;
            }
        }
        if prime * prime > n {
            return Some((n, 1));
        }
    }
    return Some((n, 1));
}

fn solve_k(k: i64) -> usize {
    let mut res = HashSet::new();
    let mut check_div = |div: i64| {
        if k % div == 0 {
            for delta in [-1, 1].iter() {
                let n = k / div + delta;
                if n + delta > 0 && (n + delta) % div == 0 {
                    let exp_prime_pow = (n + delta) / div;
                    if let Some((prime, m)) = is_prime_power(exp_prime_pow, exp_prime_pow) {
                        res.insert((n, prime, m));
                    }
                }
            }
        }
        {
            // p = 2
            for delta in [-1, 1].iter() {
                let n = 2 * k / div + delta;
                if n + delta > 0 && (n + delta) * 2 % div == 0 {
                    let exp_prime_pow = 2 * (n + delta) / div;
                    if let Some((prime, m)) = is_prime_power(exp_prime_pow, 2) {
                        res.insert((n, prime, m));
                    }
                }
            }
        }
    };
    for x in 1..=k {
        if k % x == 0 {
            check_div(x);
            check_div(k / x);
            check_div(x * 2);
            check_div(k * 2 / x);
        }
        if x * x > k {
            break;
        }
    }
    for &(n, p, m) in res.iter() {
        assert!(n > 0);
        assert!(p > 0);
        assert!(m > 0);
        // dbg!(k, n, p, m);
        assert!(n * n - k * p.pow(m as u32) == 1);
    }
    res.len()
}

fn solve(input: &mut Input, _test_case: usize) {
    out_line!(solve_k(input.read()));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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

fn slow(k: i64) -> usize {
    const MAX: i64 = 1000;
    let primes = gen_primes_table(MAX as usize);
    let mut res = 0;
    for n in 1..MAX {
        if (n * n - 1) % k == 0 {
            let exp_prime_power = (n * n - 1) / k;
            let mut ok = false;
            for p in 2..primes.len() {
                if primes[p] {
                    for pw in 1.. {
                        if p.pow(pw) as i64 == exp_prime_power {
                            ok = true;
                        }
                        if p.pow(pw) as i64 >= exp_prime_power {
                            break;
                        }
                    }
                }
            }
            if ok {
                res += 1;
            }
        }
    }
    res
}

fn stress() {
    for k in 23..200 {
        dbg!(k);
        let fast = solve_k(k);
        let correct = slow(k);
        dbg!(fast, correct);
        assert_eq!(fast, correct);
        // dbg!(solve_k(k), slow(k));
    }
}

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
