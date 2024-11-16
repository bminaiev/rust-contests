//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mid = input.usize() - 1;
        let s = input.string();
        let mut pref_sum = vec![0; n + 1];
        for i in 0..n {
            if s[i] == b'R' {
                pref_sum[i + 1] = 1;
            }
        }
        for i in 0..n {
            pref_sum[i + 1] += pref_sum[i];
        }
        let sum = |from: usize, to: usize| -> usize { pref_sum[to + 1] - pref_sum[from] };
        let cost_right = {
            let l = mid;
            let r = n - 2;
            let len = r - l + 1;
            len - sum(l, r)
        };
        let cost_left = {
            let l = 1;
            let r = mid;
            sum(l, r)
        };
        let res = cost_left.min(cost_right);
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "l";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
