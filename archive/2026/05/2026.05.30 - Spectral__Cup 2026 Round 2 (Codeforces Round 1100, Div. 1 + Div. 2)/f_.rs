#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_last_true;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let groups = input.usize();
        let mut a = input.vec::<i64>(n);
        a.sort();
        let max_val = a[a.len() - 1];
        a.pop();
        let n = n - 1;
        let sum: i64 = a.iter().sum();
        let mut dp = vec![(0, 0); 1 << n];
        let res = binary_search_last_true(0..sum + 10, |need_sum| {
            dp.fill((0, 0));
            for mask in 0..(1 << n) {
                let (cur_groups, cur_extra) = dp[mask];
                for bit in 0..n {
                    if ((1 << bit) & mask) == 0 {
                        let nmask = mask | (1 << bit);
                        let mut ngroup = cur_groups;
                        let mut nextra = cur_extra + a[bit];
                        if nextra >= need_sum {
                            ngroup += 1;
                            nextra = 0;
                        }
                        if (ngroup, nextra) > dp[nmask] {
                            dp[nmask] = (ngroup, nextra);
                        }
                    }
                }
            }
            let (cur_groups, _cur_extra) = dp[(1 << n) - 1];
            cur_groups >= groups
        })
        .unwrap_or_default();
        out.println(res + max_val);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f_";
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
