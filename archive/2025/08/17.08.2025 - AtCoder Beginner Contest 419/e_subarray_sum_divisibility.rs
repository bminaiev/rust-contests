//{"name":"E - Subarray Sum Divisibility","group":"AtCoder - AtCoder Beginner Contest 419","url":"https://atcoder.jp/contests/abc419/tasks/abc419_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5 3\n4 2 1 3\n","output":"4\n"},{"input":"7 10 4\n7 0 9 1 6 4 2\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESubarraySumDivisibility"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let l = input.usize();
    let a = input.vec::<usize>(n);
    let mut dp = vec![usize::MAX / 2; m];
    dp[0] = 0;
    for i in 0..l {
        let mut ndp = vec![usize::MAX / 2; m];
        for here in 0..m {
            let mut sum_cost = 0;
            for k in (i..n).step_by(l) {
                let now = a[k] % m;
                let cost = if now <= here {
                    here - now
                } else {
                    m - now + here
                };
                sum_cost += cost;
            }
            for prev in 0..m {
                let next = (prev + here) % m;
                ndp[next] = ndp[next].min(dp[prev] + sum_cost);
            }
        }
        dp = ndp;
    }
    out.println(dp[0]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e_subarray_sum_divisibility";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
