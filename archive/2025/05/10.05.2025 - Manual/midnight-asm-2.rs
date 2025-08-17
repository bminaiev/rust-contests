//{"name":"midnight-asm-2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"midnight-asm-2"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, prec: &[u32]) {
    let n = input.usize();
    out.println(prec[n]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1, &[]);
    output.flush();
    true
}

fn prec() -> Vec<u32> {
    const MAX_N: usize = 500;
    // dp[n vertices][balance X]
    let mut knapsack = Array2D::new(0u32, MAX_N + 1, MAX_N * 2 + 2);
    let mut tree = Array2D::new(0u32, MAX_N + 1, 3);
    knapsack[0][MAX_N] = 1;
    for n in 1..MAX_N {
        for &root_balance in [0, 2].iter() {
            for knapsack_balance in 0..=MAX_N * 2 {
                let ways = knapsack[n - 1][knapsack_balance];
                if ways == 0 {
                    continue;
                }
                let new_balance = knapsack_balance + root_balance;
                if new_balance < MAX_N {
                    continue;
                }
                let new_balance = new_balance - MAX_N;
                if new_balance > 2 {
                    continue;
                }
                tree[n][new_balance] = tree[n][new_balance].overflowing_add(ways).0;
            }
        }
        for knapsack_balance in 0..=MAX_N * 2 {
            for root_balance in 0..3 {
                for tree_size in 1..=n {
                    let knapsack_size = n - tree_size;
                    let ways_tree = tree[tree_size][root_balance];
                    let ways_knapsack = knapsack[knapsack_size][knapsack_balance];
                    let overall_ways = ways_tree.overflowing_mul(ways_knapsack).0;
                    if overall_ways == 0 {
                        continue;
                    }
                    let new_balance = knapsack_balance + root_balance;
                    if new_balance == 0 {
                        continue;
                    }
                    let new_balance = new_balance - 1;
                    if new_balance > MAX_N * 2 {
                        continue;
                    }
                    knapsack[n][new_balance] =
                        knapsack[n][new_balance].overflowing_add(overall_ways).0;
                }
            }
        }
    }
    let mut res = vec![0u32; MAX_N + 1];
    for n in 1..=MAX_N {
        for balance in 0..=2 {
            res[n] = res[n].overflowing_add(tree[n][balance]).0;
        }
    }
    res
}

fn stress() {
    let prec = prec();
    for tc in 1..=10 {
        dbg!(tc);
        let mut input = Input::new_file(format!("./tasks/midnight-asm-2/tests/{tc:02}"));
        let mut output = Output::new_file(format!("./tasks/midnight-asm-2/out/{tc:02}.out"));
        solve(&mut input, &mut output, tc, &prec);
        output.flush();
    }
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "midnight-asm-2";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
