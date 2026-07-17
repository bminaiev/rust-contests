use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};

fn gen_slow(n: usize) -> Vec<usize> {
    let mut ways = vec![0; n + 1];
    let mut a = vec![0; n];
    RecursiveFunction::new(|f, pos: usize| {
        if pos == n {
            let mut seen = vec![false; n];
            for i in 0..n {
                seen[a[i]] = true;
            }
            let mut cnt = 0;
            for i in 0..n {
                if !seen[i] {
                    cnt += 1;
                }
            }
            ways[cnt] += 1;
        } else {
            for i in 0..n {
                if i == pos {
                    continue;
                }
                a[pos] = i;
                f.call(pos + 1);
            }
        }
    })
    .call(0);
    ways
}

type Mod = Mod_998_244_353;

fn solve2(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        // [seen, alive]
        let comb = CombinationsFact::<Mod>::new(n + 1);
        let mut dp = Array2D::<Mod>::new(Mod::ZERO, n + 1, n + 1);
        dp[m][m] = comb.c(n, m);
        for seen in m..=n {
            for alive in (1..=n).rev() {
                let cur = dp[seen][alive];
                if cur == Mod::ZERO {
                    continue;
                }
                {
                    let ways_to_die = seen - 1;
                    dp[seen][alive - 1] += cur * Mod::new(ways_to_die);
                }
                if seen < n {
                    let not_seen = n - seen;
                    dp[seen + 1][alive] += cur * Mod::new(not_seen);
                }
            }
        }
        out.println(dp[n][0]);
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let comb = CombinationsFact::<Mod>::new(n + 1);
        // [component_size] -> ways to not go outside
        let mut dp = vec![Mod::ZERO; n + 1];
        for sz in m..=n {
            let mut ways = Mod::new(sz - 1).pown(sz);
            for real_sz in m..sz {
                ways -= dp[real_sz] * comb.c(sz, real_sz) * Mod::new(sz - 1).pown(sz - real_sz);
            }
            dp[sz] = ways;
        }
        let mut total_ways = Mod::new(n - 1).pown(n) * comb.c(n, m);
        for sz in m..n {
            total_ways -= dp[sz] * comb.c(n, sz) * Mod::new(n - 1).pown(n - sz) * comb.c(sz, m);
        }
        out.println(total_ways);
    }
}

fn stress() {
    for n in 1..6 {
        dbg!(n);
        dbg!(gen_slow(n));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
