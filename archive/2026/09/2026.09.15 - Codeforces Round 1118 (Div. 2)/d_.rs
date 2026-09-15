#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let cnt = [input.usize(), input.usize()];
        let mut segs = vec![vec![]; 2];
        let mut all_pos = vec![];
        for i in 0..2 {
            for _ in 0..cnt[i] {
                let l = input.i64();
                let r = input.i64() + 1;
                all_pos.push(l);
                all_pos.push(r);
                segs[i].push((l, r));
            }
        }
        all_pos.sort();
        all_pos.dedup();
        let mut dp: Vec<Vec<i64>> = vec![vec![]; all_pos.len()];
        let mut best = vec![];
        for i in 0..all_pos.len() {
            let cur = dp[i].clone();
            let pos = all_pos[i];
            if cur > best {
                best = cur.clone();
            }
            if i + 1 < all_pos.len() {
                if dp[i + 1] < cur {
                    dp[i + 1] = cur.clone();
                }
            }
            for col in 0..2 {
                for &(l, r) in segs[col].iter() {
                    if l <= pos && pos < r {
                        let mut mores_to_try = vec![r - pos];
                        {
                            // find the last segment in [1 - col], which starts before r
                            for &(l2, _r2) in segs[1 - col].iter().rev() {
                                if l2 <= r && l2 > pos {
                                    mores_to_try.push(l2 - pos);
                                    break;
                                }
                            }
                        }
                        for more in mores_to_try {
                            let mut next = cur.clone();
                            next.push(more);
                            next.sort();
                            next.reverse();
                            let next_idx = all_pos.binary_search(&(pos + more)).unwrap();
                            if next > dp[next_idx] {
                                dp[next_idx] = next;
                            }
                        }
                    }
                }
            }
            dp[i].clear();
            dp[i].shrink_to_fit();
        }
        out.println(best.len());
        out.println(best);
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
