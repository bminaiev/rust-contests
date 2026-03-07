//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::binary_search::binary_search_first_true;

type Mod = Mod_998_244_353;

fn solve_slow(max_unicorns: usize, max_tiger: usize, cats: usize) -> Mod {
    let should_go = |u: usize, t: usize| -> bool {
        let expected_win = u * 2 + t;
        let cur_money = (max_unicorns - u) * 2 + (max_tiger - t);
        expected_win as u64 > cur_money as u64 * cats as u64
    };

    let mut dp = Array2D::new(Mod::ZERO, max_unicorns + 1, max_tiger + 1);
    for u in 0..=max_unicorns {
        for t in 0..=max_tiger {
            if should_go(u, t) {
                let cnt_ok = u + t;
                let cnt_total = cnt_ok + cats;
                let mut wins = Mod::ZERO;
                if u > 0 {
                    wins += dp[u - 1][t] * Mod::new(u);
                }
                if t > 0 {
                    wins += dp[u][t - 1] * Mod::new(t);
                }
                dp[u][t] = wins / Mod::new(cnt_total);
            } else {
                dp[u][t] = Mod::new(2 * (max_unicorns - u) + (max_tiger - t));
            }
        }
    }
    dp[max_unicorns][max_tiger]
}

fn solve_fast(max_unicorns: usize, max_tiger: usize, cats: usize) -> Mod {
    let should_go = |u: usize, t: usize| -> bool {
        let expected_win = u * 2 + t;
        let cur_money = (max_unicorns - u) * 2 + (max_tiger - t);
        expected_win as u64 > cur_money as u64 * cats as u64
    };

    let comb = CombinationsFact::<Mod>::new(max_unicorns + max_tiger + 10);
    let mut still_alive = vec![Mod::ZERO; max_unicorns + max_tiger + 1];
    still_alive[0] = Mod::ONE;
    let total = max_unicorns + max_tiger + cats;
    for done_moves in 0..still_alive.len() - 1 {
        let prob_fail = Mod::new(cats) / Mod::new(total - done_moves);
        let prob_ok = Mod::ONE - prob_fail;
        still_alive[done_moves + 1] = still_alive[done_moves] * prob_ok;
    }

    let mut res = Mod::ZERO;
    for u in 0..=max_unicorns {
        let tiger_upper = binary_search_first_true(0..max_tiger, |t| should_go(u, t));
        for t in (0..=tiger_upper).rev() {
            if !should_go(u, t) {
                let mut ok = false;
                for du in 0..2 {
                    let dt = 1 - du;
                    let nu = u + du;
                    let nt = t + dt;
                    if nu <= max_unicorns && nt <= max_tiger {
                        if should_go(nu, nt) {
                            ok = true;
                            let total_moves = (max_tiger - nt) + (max_unicorns - nu);
                            let alive_here = still_alive[total_moves];
                            let total_ways = comb.c(max_tiger + max_unicorns, total_moves);
                            let good_ways = comb.c(max_tiger, max_tiger - nt)
                                * comb.c(max_unicorns, max_unicorns - nu);
                            let prob_here = alive_here * good_ways / total_ways;
                            let cards_left = nu + nt + cats;
                            let move_ways = if du == 1 { Mod::new(nu) } else { Mod::new(nt) };
                            let win_here = Mod::new(2 * (max_unicorns - u) + (max_tiger - t));

                            res += prob_here * move_ways / Mod::new(cards_left) * win_here;
                        }
                    }
                }
                if !ok {
                    break;
                }
            }
        }
    }

    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let max_unicorns = input.usize();
    let max_tiger = input.usize();
    let _panda = input.usize();
    let cats = input.usize();

    out.println(solve_fast(max_unicorns, max_tiger, cats));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "k";
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
