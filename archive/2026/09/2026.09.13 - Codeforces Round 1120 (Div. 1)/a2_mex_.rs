use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod7;
use algo_lib::math::modulo_pair::ModPair;
use algo_lib::misc::rand::Random;

type Mod = Mod7;

fn solve_case(a: &[usize]) -> Mod {
    {
        let mut all_zero = true;
        for &x in a.iter() {
            if x != 0 {
                all_zero = false;
                break;
            }
        }
        if all_zero {
            return Mod::ONE;
        }
    }
    let n = a.len();
    let mut pref = vec![0i32; n + 2];
    let mut at_most = vec![n; n + 1];
    for i in 0..n {
        let from = (i + 1) * a[i];
        let to = (i + 1) * (a[i] + 1);
        pref[from.min(n + 1)] += 1;
        pref[to.min(n + 1)] -= 1;
        for x in 0..a[i] {
            let from = (i + 1) * x;
            let to = ((i + 1) * (x + 1)).min(n);
            at_most[from] = at_most[from].min(to - 1);
            // dbg!(i + 1, from, to, at_most[from]);
        }
    }
    for i in (0..at_most.len() - 1).rev() {
        at_most[i] = at_most[i].min(at_most[i + 1]);
    }
    let mut can_use = vec![true; n + 1];
    let mut cur = 0;
    for i in 0..n {
        cur += pref[i];
        if cur != 0 {
            can_use[i] = false;
        }
    }
    // dbg!(can_use);
    // dbg!(at_most);
    // last_value -> ways to do it
    let mut dp = vec![Mod::ZERO; n + 2];
    for start in 0..=at_most[0] {
        if can_use[start] {
            dp[start] = Mod::ONE;
        }
    }
    let mut res = Mod::ZERO;
    // for i in 0..n {
    //     let cur_dp = dp[i];
    //     for j in i + 1..=at_most[i + 1] {
    //         if can_use[j] {
    //             dp[j] += cur_dp;
    //         }
    //     }
    //     if at_most[i + 1] == n {
    //         // dbg!(":!", i, cur_dp);
    //         res += cur_dp;
    //     }
    // }
    let mut extra = vec![Mod::ZERO; n + 2];
    let mut ss = Mod::ZERO;
    for i in 0..n {
        ss = ss + extra[i];
        if !can_use[i] {
            continue;
        }
        let cur_dp = dp[i] + ss;
        extra[i + 1] += cur_dp;
        extra[at_most[i + 1] + 1] -= cur_dp;
        // for j in i + 1..=at_most[i + 1] {
        //     dp[j] += cur_dp;
        // }
        if at_most[i + 1] == n {
            // dbg!(":!", i, cur_dp);
            res += cur_dp;
        }
    }
    // dbg!(dp);
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);

        let res = solve_case(&a);
        out.println(res);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..10);
        let mut hm = HashMap::<Vec<usize>, usize>::new();
        for mask in 0..(1 << n) {
            let mut vals = vec![];
            for i in 0..n {
                if ((1 << i) & mask) != 0 {
                    vals.push(i);
                }
            }
            let mut a = vec![0; n];
            for i in 0..n {
                let mut seen = vec![false; n + 1];
                for x in vals.iter() {
                    seen[x / (i + 1)] = true;
                }
                while seen[a[i]] {
                    a[i] += 1;
                }
            }
            *hm.entry(a.clone()).or_default() += 1;
        }
        for (k, v) in hm.iter() {
            let res = solve_case(k);
            if res != Mod::new(*v) {
                dbg!("????", n, k, v, res);

                // return;
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a2_mex_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "3");
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
