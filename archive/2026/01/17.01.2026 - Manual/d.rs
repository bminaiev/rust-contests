//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable4, RecursiveFunction4};

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let a = input.vec::<u32>(1 << n);
    let mut pow3 = vec![1usize; n + 1];
    for i in 1..=n {
        pow3[i] = pow3[i - 1] * 3;
    }
    let mut dp = vec![0u32; pow3[n]];
    RecursiveFunction4::new(
        |f, pos: usize, last_two: usize, mask: usize, pow2mask: usize| {
            if pos == n {
                let cur = if last_two == n {
                    a[pow2mask]
                } else {
                    let dp1 = dp[mask - pow3[last_two]];
                    let dp2 = dp[mask - pow3[last_two] * 2];
                    dp1 + dp2
                };
                dp[mask] = cur;
            } else {
                f.call(pos + 1, last_two, mask, pow2mask);
                f.call(pos + 1, last_two, mask + pow3[pos], pow2mask + (1 << pos));
                f.call(pos + 1, pos, mask + pow3[pos] * 2, pow2mask);
            }
        },
    )
    .call(0, n, 0, 0);
    let mut res = 0;
    for &val in &dp {
        res ^= val;
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
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
