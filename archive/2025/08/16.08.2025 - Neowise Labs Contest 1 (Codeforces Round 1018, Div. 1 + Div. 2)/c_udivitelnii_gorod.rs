//{"name":"C. Удивительный город","group":"Codeforces - Neowise Labs Contest 1 (Codeforces Round 1018, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2096/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 2\n2 1\n100 100\n100 100\n4\n1 2 1 2\n3 2 1 2\n1 2 1 1\n1 3 1 2\n1 2 3 4\n5 6 7 8\n3\n1 2 2\n2 2 1\n2 1 1\n100 100 100\n100 100 100\n6\n8 7 2 8 4 8\n7 7 9 7 1 1\n8 3 1 1 8 5\n6 8 3 1 1 4\n1 4 5 1 9 6\n7 1 1 6 8 2\n11 23 20 79 30 15\n15 83 73 57 34 63\n","output":"0\n14\n-1\n183\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUdivitelniiGorod"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve_1d(h: &Array2D<i64>, a: &[i64]) -> Option<i64> {
    let mut dp = [0, a[0]];
    let n = h[0].len();
    const MX: i64 = i64::MAX / 2;
    for row in 1..h.len() {
        let mut ndp = [MX; 2];
        for prev in 0..2 {
            for cur in 0..2 {
                let mut ok = true;
                for i in 0..n {
                    if h[row - 1][i] + prev as i64 == h[row][i] + cur as i64 {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ndp[cur] = ndp[cur].min(dp[prev] + a[row] * cur as i64);
                }
            }
        }
        dp = ndp;
    }
    let res = dp[0].min(dp[1]);
    if res >= MX {
        None
    } else {
        Some(res)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let h = Array2D::new_f(n, n, |_, _| input.i64());
        let a = input.vec::<i64>(n);
        let b = input.vec::<i64>(n);
        let sol1 = solve_1d(&h, &a);
        let h2 = Array2D::new_f(n, n, |i, j| h[j][i]);
        let sol2 = solve_1d(&h2, &b);
        if sol1.is_none() || sol2.is_none() {
            out.println(-1);
        } else {
            let sol1 = sol1.unwrap();
            let sol2 = sol2.unwrap();
            out.println(sol1 + sol2);
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
    const PROBLEM_NAME: &str = "c_udivitelnii_gorod";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
