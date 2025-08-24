//{"name":"B - Sort Permutation","group":"AtCoder - AtCoder Regular Contest 204 (Div. 1)","url":"https://atcoder.jp/contests/arc204/tasks/arc204_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 6 5 3 2 4\n","output":"2\n"},{"input":"1 1\n1\n","output":"0\n"},{"input":"4 6\n10 24 3 4 8 14 5 2 22 9 21 1 15 6 13 23 18 12 7 17 19 16 20 11\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::HashMap;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve_cycle(positions: &[usize], div: usize, fast: bool) -> usize {
    if fast {
        return solve_cycle_fast(positions, div);
    }
    let n = positions.len();
    // dp[start][len]
    let mut dp = Array2D::new(0, n, n + 1);
    for len in 2..=n {
        for start in 0..n {
            if fast && start + len > n {
                continue;
            }
            let mut by_color = HashMap::new();
            for delta in 0..len {
                let i = (start + delta) % n;
                by_color.entry(positions[i] % div).or_insert(vec![]).push(i);
            }
            for (_color, indexes) in by_color.iter() {
                assert!(indexes.len() <= 10);
                for mask in 1usize..(1 << indexes.len()) {
                    let mut res = mask.count_ones() as usize - 1;
                    if res == 0 {
                        continue;
                    }
                    let mut mask_indexes = vec![];
                    for i in 0..indexes.len() {
                        if mask & (1 << i) != 0 {
                            mask_indexes.push(indexes[i]);
                        }
                    }
                    for i in 0..mask_indexes.len() - 1 {
                        let p1 = mask_indexes[i];
                        let p2 = mask_indexes[i + 1];
                        let len = (p2 + n - p1) % n;
                        if len > 1 {
                            res += dp[(p1 + 1) % n][len - 1];
                        }
                    }
                    {
                        let p1 = mask_indexes[0];
                        if p1 != start {
                            let len = (n + p1 - start) % n;
                            res += dp[start][len];
                        }
                    }
                    dp[start][len] = dp[start][len].max(res);
                }
            }
        }
    }
    dp[0][n]
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ToCheck {
    positions: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    to: usize,
    res: usize,
}

fn solve_cycle_fast(positions: &[usize], div: usize) -> usize {
    let n = positions.len();
    let mut dp = Array2D::new(usize::MAX, n, n + 1);
    let mut by_color = HashMap::new();
    for i in 0..n {
        by_color.entry(positions[i] % div).or_insert(vec![]).push(i);
    }
    let mut to_check = vec![];
    let mut edges = vec![vec![]; n + 1];
    for (color, indexes) in by_color.iter() {
        for mask in 1usize..(1 << indexes.len()) {
            let mut mask_indexes = vec![];
            for i in 0..indexes.len() {
                if mask & (1 << i) != 0 {
                    mask_indexes.push(indexes[i]);
                }
            }
            to_check.push(ToCheck {
                positions: mask_indexes,
            });
        }
    }

    let mut tmp_dp = vec![0; n + 1];
    let mut calc_cost = |dp: &mut Array2D<usize>, edges: &[Vec<Edge>], from: usize, len: usize| {
        if dp[from][len] != usize::MAX {
            return dp[from][len];
        }
        tmp_dp[from + len] = 0;
        for i in (from..from + len).rev() {
            tmp_dp[i] = tmp_dp[i + 1];
            for e in edges[i].iter() {
                if e.to > from + len {
                    break;
                }
                tmp_dp[i] = tmp_dp[i].max(tmp_dp[e.to] + e.res);
            }
        }
        dp[from][len] = tmp_dp[from];
        tmp_dp[from]
    };

    to_check.sort_by_key(|t| (t.positions[t.positions.len() - 1], n - t.positions[0]));
    for to_check in to_check.iter() {
        let mut res = to_check.positions.len() - 1;
        for i in 0..to_check.positions.len() - 1 {
            let p1 = to_check.positions[i];
            let p2 = to_check.positions[i + 1];
            res += calc_cost(&mut dp, &mut edges, p1 + 1, p2 - p1 - 1);
        }

        edges[to_check.positions[0]].push(Edge {
            to: to_check.positions[to_check.positions.len() - 1] + 1,
            res,
        });
    }
    calc_cost(&mut dp, &mut edges, 0, n)
}

fn solve_case(p: &[usize], n: usize, fast: bool) -> usize {
    let mut seen = vec![false; p.len()];
    let mut res = 0;
    for start in 0..p.len() {
        if seen[p[start]] {
            continue;
        }
        let mut positions = vec![];
        let mut pos = start;
        while !seen[p[pos]] {
            positions.push(p[pos]);
            seen[p[pos]] = true;
            pos = p[pos];
        }
        res += solve_cycle(&positions, n, fast);
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let k = input.usize();
    let p = input.vec::<usize>(n * k).sub_from_all(1);
    let res = solve_case(&p, n, true);
    out.println(res);
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..40);
        let k = rnd.gen(1..10);
        let p = rnd.gen_permutation(n);
        let res1 = solve_case(&p, (n + k - 1) / k, false);
        let res2 = solve_case(&p, (n + k - 1) / k, true);
        if res1 != res2 {
            dbg!(&p, res1, res2);
            break;
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
    const PROBLEM_NAME: &str = "b_sort_permutation";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
