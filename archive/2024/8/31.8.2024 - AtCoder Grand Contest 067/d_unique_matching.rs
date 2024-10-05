//{"name":"D - Unique Matching","group":"AtCoder - AtCoder Grand Contest 067","url":"https://atcoder.jp/contests/agc067/tasks/agc067_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1005488041\n","output":"6\n"},{"input":"5 1005488041\n","output":"102960\n"},{"input":"100 1005488041\n","output":"47599495\n"},{"input":"1000 1005488041\n","output":"632708165\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUniqueMatching"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo_runtime::ModRuntime;

type Mod = ModRuntime;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.i32();

    // cnt currently open
    let mut dp = vec![Mod::new(0, m); n + 1];
    dp[0] = Mod::new(1, m);

    let mut c = Array2D::new(Mod::new(0, m), n + 1, n + 1);
    c[0][0] = Mod::new(1, m);
    for i in 1..=n {
        c[i][0] = Mod::new(1, m);
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    // let mut fact = vec![Mod::new(0, m); n + 1];
    // fact[0] = Mod::new(1, m);
    // for i in 1..=n {
    //     fact[i] = fact[i - 1] * Mod::new(i as i32, m);
    // }

    for pos in 0..n {
        let mut new_dp = vec![Mod::new(0, m); n + 1];
        for cnt_open in 0..=n - pos {
            let cur = dp[cnt_open];
            let left = n - pos - cnt_open;
            for cnt_new_open in 0..=left {
                let total_open = cnt_open + cnt_new_open;
                if total_open == 0 {
                    continue;
                }
                assert!(total_open <= n);
                let cur = cur * c[left][cnt_new_open];

                if total_open == 1 {
                    let end_ways = Mod::new((n - pos) as i32, m);
                    new_dp[0] += cur * end_ways;
                } else {
                    new_dp[total_open - 1] += cur * Mod::new(total_open as i32, m);
                }
            }
        }
        dp = new_dp;
    }
    let res = dp[0];
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_unique_matching";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
