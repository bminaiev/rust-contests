//{"name":"E. Скупая сетка и подсчёты","group":"Codeforces - Order Capital Round 1 (Codeforces Round 1038, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2122/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n4 3\n2 1 -1 2\n2 -1 1 3\n5 4\n1 3 -1 4 2\n-1 3 4 2 -1\n10 10\n-1 -1 -1 -1 -1 -1 -1 -1 -1 -1\n-1 -1 -1 -1 -1 -1 -1 -1 -1 -1\n","output":"6\n64\n123782927\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESkupayaSetkaIPodschyoti"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.i32();
        let a = [input.vec::<i32>(n), input.vec::<i32>(n)];
        let mut dp = vec![Mod::ZERO; k as usize + 2];
        let K = Mod::new(k);
        // dp[0] == no commitment
        // dp[1] == diff = 0, but need to be down
        // dp[x] == diff = x - 1
        if a[0][0] == -1 {
            dp[0] = K;
        } else {
            dp[0] = Mod::ONE;
        }
        for i in 0..(n - 1) {
            let mut ndp = vec![Mod::ZERO; k as usize + 2];

            for cur_delta in 0..=k + 1 {
                let cur_delta = cur_delta as usize;
                let ways = dp[cur_delta];
                for down in 1..=k {
                    for right in 1..=k {
                        if a[1][i] != -1 && a[1][i] != down {
                            continue;
                        }
                        if a[0][i + 1] != -1 && a[0][i + 1] != right {
                            continue;
                        }
                        if down > right {
                            let new_delta = (down - right) as usize + 1;
                            ndp[new_delta] += ways;
                        } else if down == right {
                            ndp[cur_delta] += ways;
                        } else {
                            let diff = (right - down) as usize;
                            if cur_delta == 0 {
                                ndp[cur_delta] += ways;
                            } else if cur_delta > diff {
                                let new_delta = cur_delta - diff;
                                ndp[new_delta] += ways;
                            }
                        }
                    }
                }
            }
            dp = ndp;
        }
        let mut res = Mod::ZERO;
        for delta in 0..=k + 1 {
            let delta = delta as usize;
            res += dp[delta];
        }
        if a[1][n - 1] == -1 {
            res *= K;
        }
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
    const PROBLEM_NAME: &str = "e_skupaya_setka_ipodschyoti";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
