//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn calc_f32(max_a: usize, max_b: usize) -> f32 {
    let mut dp_prev = vec![0.0f32; max_b + 1];
    let mut dp = vec![0.0f32; max_b + 1];
    for cur_a in 1..=max_a {
        dp[0] = cur_a as f32;
        for cur_b in 1..=max_b {
            let pa = cur_a as f32 / (cur_a as f32 + cur_b as f32);
            let pb = cur_b as f32 / (cur_a as f32 + cur_b as f32);
            dp[cur_b] = pa * (1.0 + dp_prev[cur_b]) + pb * (-1.0 + dp[cur_b - 1]);
            if dp[cur_b] < 0.0 {
                dp[cur_b] = 0.0;
            }
        }
        std::mem::swap(&mut dp, &mut dp_prev);
    }
    dp_prev[max_b]
}

fn calc(max_a: usize, max_b: usize, buben: usize) -> f64 {
    let mut dp_prev = vec![0.0f64; max_b + 1];
    let mut dp = vec![0.0f64; max_b + 1];
    let mut first_negative = 1;
    for cur_a in 1..=max_a {
        dp[0] = cur_a as f64;
        let expected_b = (max_b as f64 / max_a as f64 * cur_a as f64) as usize;
        let from_b = if expected_b > buben {
            expected_b - buben
        } else {
            1
        };
        for cur_b in from_b..first_negative {
            let pa = cur_a as f64 / (cur_a as f64 + cur_b as f64);
            let pb = cur_b as f64 / (cur_a as f64 + cur_b as f64);
            dp[cur_b] = pa * (1.0 + dp_prev[cur_b]) + pb * (-1.0 + dp[cur_b - 1]);
        }
        while first_negative <= max_b && first_negative < expected_b + buben {
            let cur_b = first_negative;
            let pa = cur_a as f64 / (cur_a as f64 + cur_b as f64);
            let pb = cur_b as f64 / (cur_a as f64 + cur_b as f64);
            dp[cur_b] = pa * (1.0 + dp_prev[cur_b]) + pb * (-1.0 + dp[cur_b - 1]);
            if dp[cur_b] < 0.0 {
                dp[cur_b] = 0.0;
                break;
            }
            first_negative += 1;
        }
        std::mem::swap(&mut dp, &mut dp_prev);
    }
    dp_prev[max_b]
}

const BUBEN: usize = 4000;

fn solve(input: &mut Input, out: &mut Output) {
    let max_a = input.usize();
    let max_b = input.usize();
    // let max_a = 30_000;
    // let max_b = 30_000;
    let result = calc(max_a, max_b, BUBEN);
    out.println(result);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

fn stress() {
    const MAX_A: usize = 100_000;
    const MAX_B: usize = 50_000;
    for _ in 0..10 {
        let start = Instant::now();
        let zz = calc(MAX_A, MAX_B, BUBEN);
        dbg!(start.elapsed());
        let zz2 = calc(MAX_A, MAX_B, BUBEN * 2);
        dbg!(zz, zz2);
    }
    // const MAX_N: usize = 8;
    // for a in 1..=MAX_N {
    //     let mut row = vec![];
    //     for b in 1..=MAX_N {
    //         let str = format!("{} ", calc_f32(a, b));
    //         row.push(str);
    //     }
    //     dbg!(a, row);
    // }
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
