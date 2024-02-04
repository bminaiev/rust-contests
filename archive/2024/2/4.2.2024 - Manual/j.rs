//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::gcd::{gcd, lcm};
use algo_lib::math::primes::{gen_primes_table, is_prime};
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {}

fn stress() {
    let n = 100;
    let mut rnd = Random::new(7877881);
    let mut perm = rnd.gen_permutation(n);
    for x in perm.iter_mut() {
        *x += 1;
    }
    let mut pairs = vec![];
    for i in 0..n {
        for j in i + 1..n {
            pairs.push((i, j));
        }
    }
    rnd.shuffle(&mut pairs);
    let mut cnt = 0;
    let mut cur_lcm = vec![1; n];
    let mut primes_table = gen_primes_table(n);
    primes_table[1] = true;
    let mut prime_iter = 2;
    let mut ss = 0;
    for x in primes_table.iter() {
        if *x {
            ss += 1;
        }
    }
    dbg!(ss);
    let mut prev_ss = 0;
    for (i, j) in pairs {
        if !primes_table[cur_lcm[i]] && !primes_table[cur_lcm[j]] {
            continue;
        }
        let mut cur_ss = 0;
        for x in cur_lcm.iter() {
            if primes_table[*x] {
                cur_ss += 1;
            }
        }
        if cur_ss != prev_ss {
            prev_ss = cur_ss;
            dbg!(cur_ss, cnt);
        }
        cnt += 1;
        let g = gcd(perm[i], perm[j]);
        cur_lcm[i] = lcm(cur_lcm[i], g);
        cur_lcm[j] = lcm(cur_lcm[j], g);
        while prime_iter < primes_table.len() && prime_iter < 125 {
            let mut good = vec![];
            for i in 0..n {
                if cur_lcm[i] % prime_iter == 0 {
                    good.push(i);
                }
            }
            if good.is_empty() {
                break;
            }
            eprintln!("prime_iter = {}. cnt = {cnt}", prime_iter);
            let v = good[rnd.gen(0..good.len())];
            for i in 0..n {
                if primes_table[cur_lcm[i]] && cur_lcm[i] % prime_iter != 0 {
                    let g = gcd(perm[i], perm[v]);
                    let next_lcm = lcm(cur_lcm[i], g);
                    if next_lcm > cur_lcm[i] {
                        dbg!(i, cur_lcm[i], next_lcm, perm[i]);
                        cur_lcm[i] = next_lcm;
                    }
                    cnt += 1;
                }
            }

            prime_iter += 1;
            while prime_iter < primes_table.len() && !primes_table[prime_iter] {
                prime_iter += 1;
            }
        }
    }
    for i in 0..n {
        let prime = primes_table[perm[i]];
        let my_prime = primes_table[cur_lcm[i]];
        if prime != my_prime {
            dbg!(perm[i], cur_lcm[i]);
        }
    }
    dbg!(cnt);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "j";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
