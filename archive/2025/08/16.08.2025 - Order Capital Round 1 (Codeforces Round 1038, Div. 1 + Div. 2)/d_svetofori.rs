//{"name":"D. Светофоры","group":"Codeforces - Order Capital Round 1 (Codeforces Round 1038, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2122/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n6 6\n1 2\n2 3\n3 4\n4 6\n1 5\n5 6\n4 3\n1 2\n1 3\n1 4\n","output":"4 2\n3 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSvetofori"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let u = input.usize() - 1;
            let v = input.usize() - 1;
            g[u].push(v);
            g[v].push(u);
        }
        // dp[v] -> min wait
        let mut dp = vec![usize::MAX / 2; n];
        dp[0] = 0;
        for t in 0.. {
            if dp[n - 1] < usize::MAX / 2 {
                out.println(vec![t, dp[n - 1]]);
                break;
            }
            let mut ndp = vec![usize::MAX / 2; n];
            for i in 0..n {
                ndp[i] = dp[i] + 1;
            }
            for i in 0..n {
                let nxt = g[i][t % g[i].len()];
                if ndp[nxt] > dp[i] {
                    ndp[nxt] = dp[i];
                }
            }
            dp = ndp;
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_svetofori";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
