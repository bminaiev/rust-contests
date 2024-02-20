//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut pos_of = vec![usize::MAX; n];
    for i in 0..n {
        pos_of[a[i]] = i;
    }
    let mut left_more = vec![0; n];
    let mut fenw = Fenwick::new(n);
    for i in (0..n).rev() {
        left_more[i] = fenw.get_sum(pos_of[i]);
        fenw.add(pos_of[i], 1);
    }
    let mut dp = vec![i64::MAX; n + 1];
    dp[0] = 0;
    for done in 0..n {
        {
            let cur = dp[done] + left_more[done];
            dp[done + 1] = dp[done + 1].min(cur);
        }
        if done + 2 <= n && pos_of[done] > pos_of[done + 1] {
            let cur = dp[done] + left_more[done] + left_more[done + 1] - 1;
            dp[done + 2] = dp[done + 2].min(cur);
        }
    }
    out.println(dp[n]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
