//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let cnk = CombinationsFact::new(n + 1);
        let mut p = vec![0];
        let mut g = vec![vec![]; n];
        for i in 1..n {
            let x = input.usize() - 1;
            g[x].push(i);
            p.push(x);
        }
        let mut dp_down0 = vec![0; n];
        let mut dp_down1 = vec![0; n];
        let mut dp_down2 = vec![0; n];
        let mut dp = vec![0; n];
        let mut dp1 = vec![0; n];
        // let mut dp2 = vec![0; n];
        for v in 0..n {
            dp_down0[v] = 1;
            dp[v] = 1;
        }
        let mut res = Mod::ZERO;
        if k == 1 {
            res = Mod::new(n);
        } else {
            for d in 1.. {
                for v in 0..n {
                    dp_down2[v] = dp_down1[v];
                    dp_down1[v] = dp_down0[v];
                    dp_down0[v] = 0;
                    // dp2[v] = dp1[v];
                    dp1[v] = dp[v];
                    dp[v] = 0;
                }
                for v in 1..n {
                    dp_down0[p[v]] += dp_down1[v];
                }
                // dbg!(d);
                for v in 1..n {
                    dp[v] = dp_down0[v] + dp1[p[v]] - dp_down2[v];
                }
                dp[0] = dp_down0[0];
                // dbg!(d, &dp, &dp_down0);
                let mut finished = true;
                for v in 0..n {
                    if dp[v] != 0 {
                        finished = false;
                    }
                    if dp[v] >= k - 1 {
                        res += cnk.c(dp[v], k - 1);
                    }
                }
                if finished {
                    break;
                }
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
