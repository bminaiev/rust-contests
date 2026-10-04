use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let mut p = input.vec::<usize>(n - 1).sub_from_all(1);
    p.insert(0, 0);
    let mut cnt = vec![0; n];
    for v in 1..n {
        cnt[p[v]] += 1;
    }
    let mx = n + m;
    // dp[leafs][chosen]
    let mut dp = Array2D::new(Mod::ZERO, mx + 1, mx + 1);
    {
        // tmp_dp[chosen]
        let mut tmp_dp = vec![Mod::ZERO; mx + 1];
        tmp_dp[0] = Mod::ONE;
        for v in 0..n {
            let mut ndp = vec![Mod::ZERO; mx + 1];
            for chosen in 0..=mx {
                let cur = tmp_dp[chosen];
                if cur == Mod::ZERO {
                    continue;
                }
                ndp[chosen] += cur;
                if cnt[v] > 0 {
                    ndp[chosen + 1] += cur * Mod::new(cnt[v]);
                }
            }
            tmp_dp = ndp;
        }
        let mut cnt_leafs = 0;
        for v in 0..n {
            if cnt[v] == 0 {
                cnt_leafs += 1;
            }
        }
        for chosen in 0..=mx {
            dp[cnt_leafs][chosen] = tmp_dp[chosen];
        }
    }
    for added in 0..m {
        let cur_size = n + added;
        let mut ndp = Array2D::new(Mod::ZERO, mx + 1, mx + 1);
        for leafs in 0..=mx {
            for chosen in 0..=mx {
                let cur = dp[leafs][chosen];
                if cur == Mod::ZERO {
                    continue;
                }
                {
                    // add to leaf
                    ndp[leafs][chosen] += cur * Mod::new(leafs);
                    ndp[leafs][chosen + 1] += cur * Mod::new(leafs);
                }
                {
                    // add to chosen
                    ndp[leafs + 1][chosen] += cur * Mod::new(chosen);
                }
                {
                    // add to not chosen
                    assert!(cur_size >= leafs + chosen);
                    let cnt_not_chosen = cur_size - leafs - chosen;
                    {
                        // choose it!
                        ndp[leafs + 1][chosen + 1] += cur * Mod::new(cnt_not_chosen);
                    }
                    {
                        // don't choose it
                        ndp[leafs + 1][chosen] += cur * Mod::new(cnt_not_chosen);
                    }
                }
            }
        }
        dp = ndp;
    }
    let mut res = Mod::ZERO;
    for leafs in 1..=mx {
        let chosen = n + m - leafs;
        res += dp[leafs][chosen] / Mod::new(leafs);
    }
    for added in 0..m {
        res /= Mod::new(n + added);
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_astana_hard_metro";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
