use std::sync::atomic::AtomicI32;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::{collections::array_2d::Array2D, io::output};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn calc_score(s: &[u8], mask: u32, m: usize, all_submasks: &[Vec<usize>]) -> usize {
    let n = s.len() + 1;
    let mut dsu = Dsu::new(n);
    for i in 0..n - 1 {
        if s[i] == b'C' || ((1 << i) & mask) != 0 {
            dsu.unite(i, i + 1);
        }
    }
    let mut cur_score = 0;
    for to_check in all_submasks.iter() {
        let mut cnt_comps = m;
        if m >= 2 {
            if dsu.get(to_check[0]) == dsu.get(to_check[1]) {
                cnt_comps -= 1;
            }
            if m >= 3 {
                if dsu.get(to_check[0]) == dsu.get(to_check[2]) {
                    cnt_comps -= 1;
                }
                if cnt_comps > 1 && dsu.get(to_check[1]) == dsu.get(to_check[2]) {
                    cnt_comps -= 1;
                }
            }
        }
        if m == 2 {
            if cnt_comps == 2 {
                cur_score += 1;
            } else {
                cur_score += 2;
            }
        } else if m == 3 {
            if cnt_comps == 1 {
                cur_score += 6;
            } else if cnt_comps == 3 {
                cur_score += 1;
            } else {
                cur_score += 2;
            }
        }
    }
    cur_score
}

fn gen_all_submasks(n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut all_submasks = vec![];
    for mask in 0usize..(1 << n) {
        if mask.count_ones() as usize == m {
            let mut idxs = vec![];
            for i in 0..n {
                if ((1 << i) & mask) != 0 {
                    idxs.push(i);
                }
            }
            all_submasks.push(idxs);
        }
    }
    all_submasks
}

fn solve_case(s: &[u8], more: usize, m: usize) -> Vec<u8> {
    // let more = more.min(100);
    let n = s.len() + 1;
    // dp[seen][budget_more]
    let mut dp = Array2D::new(0, n + 1, more + 1);
    // (used, len)
    let mut dp_prev = Array2D::new((false, 1u32, 0u16), n + 1, more + 1);
    for i in 0..=n {
        for j in 0..=more {
            dp_prev[i][j] = (false, 1, j as u16);
        }
    }
    for seen in 0..n {
        for budget in 0..=more {
            let cur_value = dp[seen][budget];
            {
                // skip here
                if cur_value > dp[seen + 1][budget] {
                    dp[seen + 1][budget] = cur_value;
                    dp_prev[seen + 1][budget] = (false, 1, budget as u16);
                }
            }
            let mut cost = 0;
            for len in 2..=(n - seen) {
                if s[seen + len - 2] != b'C' {
                    cost += 1;
                }
                if cost > budget {
                    break;
                }
                let nbudget = budget - cost;
                let mut res = cur_value;
                if m == 2 {
                    res += len * (len - 1);
                } else if m == 3 {
                    if len >= 3 {
                        res += len * (len - 1) * (len - 2) * 5;
                    }
                    res += len * (len - 1) * (n - len) * 3;
                }
                if res > dp[seen + len][nbudget] {
                    dp[seen + len][nbudget] = res;
                    dp_prev[seen + len][nbudget] = (true, len as u32, budget as u16);
                }
            }
        }
    }
    // dbg!(dp[2][1]);

    let mut res = s.to_vec();

    let mut cur_seen = n;
    let mut cur_budget = 0;
    for budget in 1..=more {
        if dp[cur_seen][budget] > dp[cur_seen][cur_budget] {
            cur_budget = budget;
        }
    }

    while cur_seen != 0 {
        let (used, len, budget) = dp_prev[cur_seen][cur_budget];
        cur_budget = budget as usize;
        cur_seen -= len as usize;
        if used {
            for i in 0..len as usize - 1 {
                res[cur_seen + i] = b'C';
            }
        }
    }
    assert!(cur_budget <= more);

    res
}

fn solve_case_slow(s: &[u8], more: usize, m: usize) -> Vec<u8> {
    let n = s.len() + 1;
    let mut best_mask = 0;
    let mut best_sum = 0;
    let all_submasks = gen_all_submasks(n, m);
    for mask in 0u32..(1 << (n - 1)) {
        if mask.count_ones() as usize != more {
            continue;
        }
        let cur_score = calc_score(s, mask, m, &all_submasks);
        if cur_score > best_sum {
            best_sum = cur_score;
            best_mask = mask;
        }
    }
    let mut res = s.to_vec();
    for i in 0..n - 1 {
        if ((1 << i) & best_mask) != 0 {
            res[i] = b'C';
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let mut inputs = vec![];
    for tt in 0..tc {
        let n = input.usize();
        let more = input.usize();
        let m = input.usize();
        let s = input.string();
        inputs.push((tt, s, more, m));
    }
    let mut solved = AtomicI32::new(0);
    let outputs: Vec<_> = inputs
        .into_par_iter()
        .map(|(tt, s, more, m)| {
            let res = solve_case(&s, more, m);
            let solved = solved.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            dbg!(solved, tc, s.len(), more, m);
            res
        })
        .collect();
    for s2 in outputs {
        let str = String::from_utf8(s2).unwrap();
        out.println(str);
    }
}

fn my_score(s: &[u8], m: usize) -> usize {
    let mut i = 0;
    let n = s.len() + 1;
    let mut res = 0;
    while i != s.len() {
        if s[i] == b'.' {
            i += 1;
            continue;
        }
        let mut j = i;
        while j != s.len() && s[j] == b'C' {
            j += 1;
        }
        let len = j - i + 1;
        if m == 2 {
            res += len * (len - 1);
        } else if m == 3 {
            if len >= 3 {
                res += len * (len - 1) * (len - 2) * 5;
            }
            res += len * (len - 1) * (n - len) * 3;
        }
        i = j;
    }
    res
}

fn stress() {
    let n = 20;
    let m = 3;
    let all_submasks = gen_all_submasks(n, m);

    for count_ones in 1..n {
        let mut all_ss = vec![];
        dbg!(count_ones);
        for mask in 0usize..(1 << (n - 1)) {
            if mask.count_ones() as usize != count_ones {
                continue;
            }
            let mut s = vec![0; n - 1];
            for i in 0..n - 1 {
                if ((1 << i) & mask) != 0 {
                    s[i] = 1;
                }
            }
            let s_bytes: Vec<u8> = s
                .iter()
                .map(|&x| if x == 1 { b'C' } else { b'.' })
                .collect();
            // let score = calc_score(&s_bytes, 0, m, &all_submasks);
            // dbg!(score);
            all_ss.push(s_bytes);
        }

        all_ss.sort_by_key(|ss| my_score(ss, m));
        let mut max_seen = 0;
        for ss in all_ss.iter() {
            let score = calc_score(&ss, 0, m, &all_submasks);
            // dbg!(ss, score);
            assert!(score >= max_seen);
            max_seen = score;
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "p";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
    run_dragon(solve, "P", 3..4);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
