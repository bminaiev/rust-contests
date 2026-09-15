use std::collections::BTreeSet;
use std::unreachable;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::gcd::{gcd, lcm};
use algo_lib::math::primes::{factorize, gen_largest_prime_table};
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};

type SegTree = SegTreeMax<usize>;

fn solve_case(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let largest_primes = gen_largest_prime_table(2 * n + 10);
    let mut primes_powers = vec![];
    for x in 2..largest_primes.len() {
        let fact: Vec<_> = factorize(&largest_primes, x).collect();
        if fact.len() == 1 {
            primes_powers.push(x);
        }
    }
    let mut st = SegTree::new(primes_powers.len(), |pos| MaxValNode { max_val: n, pos });
    let mut exist = vec![false; primes_powers.len()];
    for left in (0..n).rev() {
        for prime in factorize(&largest_primes, a[left]) {
            let mut real_power = 1;
            for _pw in 1..=prime.power {
                real_power *= prime.value;
                let idx = primes_powers.binary_search(&real_power).unwrap();
                let right = st.get(idx..idx + 1).max_val;
                if idx == 0 || st.get(0..idx).max_val < right {
                    if right != left + 1 {
                        exist[idx] = true;
                    }
                }
            }
        }
        for prime in factorize(&largest_primes, a[left]) {
            let mut real_power = 1;
            for _pw in 1..=prime.power {
                real_power *= prime.value;
                let idx = primes_powers.binary_search(&real_power).unwrap();
                st.update_point(
                    idx,
                    MaxValNode {
                        max_val: left,
                        pos: idx,
                    },
                );
            }
        }
    }
    for i in 0..primes_powers.len() {
        let right = st.get(i..i + 1).max_val;
        if i == 0 || st.get(0..i).max_val < right {
            if right != 0 {
                exist[i] = true;
            }
        }
    }
    let mut res = vec![];
    for i in 0..exist.len() {
        if exist[i] {
            res.push(primes_powers[i]);
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);
        let res = solve_case(&a);
        out.println(res.len());
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

fn solve_slow(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut res = BTreeSet::new();
    for l in 0..n {
        for r in l + 1..=n {
            for test in 2.. {
                let mut cur_test = 1;
                for i in l..r {
                    cur_test = lcm(cur_test, gcd(a[i], test));
                }
                if cur_test != test {
                    res.insert(test);
                    break;
                }
            }
        }
    }
    res.into_iter().collect()
}

fn stress() {
    for it in 354.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(2..50);
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = rnd.gen_range(1..n);
        }
        let res = solve_case(&a);
        let res_slow = solve_slow(&a);
        if res != res_slow {
            dbg!(a, res, res_slow);
            unreachable!();
        }
    }
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e_divmex";
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
