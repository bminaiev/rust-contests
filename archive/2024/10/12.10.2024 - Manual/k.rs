//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    // type M = Mod_998_244_353;
    // let x = M::new(831870305);
    // dbg!(x);

    let n = input.usize();
    let a = input.vec::<usize>(n - 1);
    const MAX: usize = 100_000 + 5;
    let mut dp = vec![0; MAX];
    for i in 1..MAX {
        dp[i] = i as i64;
    }
    for &d in a.iter() {
        let mut dp_up = vec![i64::MAX / 2; MAX];
        for i in 0..MAX - d {
            dp_up[i + d] = dp_up[i + d].min(dp[i] + d as i64);
        }
        for i in 0..MAX - 1 {
            dp_up[i + 1] = dp_up[i + 1].min(dp_up[i] + 1);
        }
        let mut dp_down = vec![i64::MAX / 2; MAX];
        for i in d..MAX {
            dp_down[i - d] = dp_down[i - d].min(dp[i] as i64);
        }
        for i in (1..MAX).rev() {
            dp_down[i - 1] = dp_down[i - 1].min(dp_down[i]);
        }
        for i in 0..MAX {
            dp[i] = dp_up[i].min(dp_down[i]);
        }
    }
    let res = *dp.iter().min().unwrap();
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "k";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
