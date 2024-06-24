//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let cost = input.usize();
    let a = input.vec::<usize>(n);

    let comb: CombinationsFact<Mod> = CombinationsFact::new(n + 1);

    let tot_sum = a.iter().sum::<usize>();
    let mut dp = Array2D::new(Mod::ZERO, n + 1, tot_sum + 1);
    dp[0][0] = Mod::ONE;
    for &x in a.iter() {
        for used in (0..=n).rev() {
            for sum in (0..=tot_sum).rev() {
                let cur = dp[used][sum];
                if cur == Mod::ZERO {
                    continue;
                }
                dp[used + 1][sum + x] += cur;
            }
        }
    }
    let mut res = Mod::ZERO;
    // let all_ways_inv = Mod::ONE / comb.fact(n);
    for used in 0..n {
        for sum in 0..=tot_sum {
            let cur = dp[used][sum];
            let left = n - used;
            let sum_left = tot_sum - sum;
            let prob = Mod::ONE / comb.c(n, used);
            let cur_cost = if cost * left < sum_left {
                Mod::new(cost)
            } else {
                Mod::new(sum_left) / Mod::new(left)
            };
            res += prob * cur_cost * cur;
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
