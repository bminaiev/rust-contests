//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let k = input.usize();
    let max_pos = n - k + 1;
    let mut dp = vec![usize::MAX - 1; 1 << max_pos];
    let mut dp_next_mask = vec![0; 1 << max_pos];
    dp[0] = 0;
    let mut best_move = vec![usize::MAX; 1 << max_pos];
    let full_mask = (1 << max_pos) - 1;
    loop {
        let mut ch = false;
        for mask in 0..(1 << max_pos) {
            // dbg!(mask);
            for p in 0..max_pos {
                let mut nmask = mask;

                let mut from = 0;
                if p >= k - 1 {
                    from = p + 1 - k;
                }
                for i in from..=p {
                    nmask &= !(1 << i);
                }
                nmask |= (nmask >> 1) | (nmask << 1);
                nmask &= full_mask;
                // dbg!(mask, p, nmask);
                if dp[nmask] + 1 < dp[mask] {
                    dp[mask] = dp[nmask] + 1;
                    // dbg!(mask, dp[mask]);
                    best_move[mask] = p;
                    dp_next_mask[mask] = nmask;
                    ch = true;
                }
            }
        }
        if !ch {
            break;
        }
    }
    // dbg!(full_mask);
    assert!(dp[full_mask] < 20);
    let mut cur_mask = full_mask;
    loop {
        let p = best_move[cur_mask];
        assert!(p != usize::MAX);
        let zz = vec![p + 1, p + 1];
        out.println(zz);
        out.flush();
        let res = input.usize();
        if res == 1 {
            break;
        }
        cur_mask = dp_next_mask[cur_mask];
    }
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
