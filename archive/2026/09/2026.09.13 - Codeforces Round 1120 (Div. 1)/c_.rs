use std::collections::{BTreeSet, HashMap};
use std::unreachable;

use algo_lib::collections::permutation::Permutation;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::CombinationsFact;
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::rand::Random;

type Mod = Mod7;

fn good_vals(a: &[usize]) -> bool {
    let n = a.len();
    for start in 0..n {
        let mut cur = start;
        let mut seen = vec![false; n];
        seen[cur] = true;
        for _ in 0..n - 1 {
            let mx = a[cur..].iter().max().unwrap();
            let need = mx - a[cur];
            let mut pos = n;
            for i in 0..n {
                if a[i] == need {
                    pos = i;
                }
            }
            if pos == n {
                break;
            }
            seen[pos] = true;
            cur = pos;
        }
        if seen.iter().all(|x| *x) {
            return true;
        }
    }
    false
}

fn solve_case_slow(a: &[usize]) -> Mod {
    let mut res = Mod::ZERO;
    let mut perm = Permutation::new(a.len());

    let n = a.len();
    let mut res = Mod::ZERO;
    loop {
        let mut vals = vec![0; n];
        for i in 0..n {
            vals[i] = a[perm[i]];
        }
        if good_vals(&vals) {
            // dbg!(vals);
            res += Mod::ONE;
        }
        if !perm.next() {
            break;
        }
    }
    res
}

fn solve_case(a: &[usize]) -> Mod {
    let mut res = Mod::ZERO;
    let n = a.len();
    let mx = a[n - 1];

    let facts = CombinationsFact::<Mod>::new(n + 1);

    for &last in a[n - 3..n - 1].iter() {
        let mut alive: BTreeSet<usize> = a.iter().copied().collect();
        alive.remove(&0);
        alive.remove(&last);
        alive.remove(&mx);
        let mut cur = last;
        for i in 0..n - 3 {
            let need = if i % 2 == 1 { last - cur } else { mx - cur };
            // dbg!(need);
            if !alive.remove(&need) {
                break;
            }
            cur = need;
        }
        if alive.is_empty() {
            let half = n / 2;
            res += facts.fact(half) * facts.fact(n - half - 2);
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);
        out.println(solve_case(&a));
    }
}

fn stress() {
    let mut answer = HashMap::<usize, BTreeSet<Mod>>::new();
    for it in 42..5000 {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..30);
        let mut a = vec![0];
        let mut p = rnd.gen_double();
        for i in 1..n {
            if rnd.gen_double() < p {
                a.push(i);
            }
        }
        let nn = a.len();
        if nn < 4 || n >= 10 {
            continue;
        }
        // if a.len() != 7 {
        //     continue;
        // }
        let res = solve_case_slow(&a);
        let res2 = solve_case(&a);
        answer.entry(a.len()).or_default().insert(res);
        dbg!(a.len(), res);
        dbg!(answer);
        if res != res2 {
            dbg!(a, res, res2);
            unreachable!();
        }
        // if *a.iter().max().unwrap() > 10 && res != Mod::ZERO {
        //     break;
        // }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_";
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
