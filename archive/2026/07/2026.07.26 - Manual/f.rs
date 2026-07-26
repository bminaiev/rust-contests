use std::collections::{HashSet, VecDeque};

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::rand::Random;

type Mod = Mod7;

fn solve_stupid(g: &[Vec<usize>]) -> Mod {
    let n = g.len();
    let mut queue = VecDeque::new();
    //     for each node x: visited[x] = false

    // steps = 0
    // queue.push(start)
    // visited[start] = true

    // while not queue.empty():
    //     steps += 1
    //     current = queue.pop_front()
    //     for next in neighbors(current):
    //         visited[next] = true
    //         queue.push(next)
    let mut visited = vec![false; n];
    let mut res = Mod::ZERO;
    queue.push_back(0);
    visited[0] = true;
    let mut last_steps = Mod::ZERO;
    while let Some(v) = queue.pop_front() {
        res += Mod::ONE;
        if res == Mod::new(20_00_000) {
            break;
        }
        for &to in &g[v] {
            if !visited[to] {
                visited[to] = true;
                last_steps = res;
                queue.push_back(to);
            } else {
                queue.push_back(to);
            }
        }
    }
    last_steps
}

fn solve_case(g: &[Vec<usize>]) -> Mod {
    let n = g.len();
    let mut queue_by_lvl = vec![vec![]; n + 1];
    queue_by_lvl[0].push(0);
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut prev_idx = vec![usize::MAX; n];
    for lvl in 0..=n {
        for i in 0..queue_by_lvl[lvl].len() {
            let v = queue_by_lvl[lvl][i];
            for &to in &g[v] {
                if !visited[to] {
                    visited[to] = true;
                    queue_by_lvl[lvl + 1].push(to);
                    prev_idx[to] = i;
                }
            }
        }
    }
    let mut last_layer = 0;
    for lvl in 1..=n {
        if !queue_by_lvl[lvl].is_empty() {
            last_layer = lvl;
        }
    }
    if last_layer == 0 {
        return Mod::ZERO;
    }
    let mut last_path_index = vec![0; n];
    last_path_index[last_layer] = queue_by_lvl[last_layer].len() - 1;
    for lvl in (1..last_layer).rev() {
        last_path_index[lvl] = prev_idx[queue_by_lvl[lvl + 1][last_path_index[lvl + 1]]];
    }
    // dp[v][len] = how many paths, starting from v, of length len.
    let mut dp = Array2D::new(Mod::ZERO, n + 1, last_layer + 1);
    for v in 0..n {
        dp[v][0] = Mod::ONE;
    }
    for len in 1..=last_layer {
        for v in 0..n {
            let mut sum = Mod::ONE;
            for &to in &g[v] {
                sum += dp[to][len - 1];
            }
            dp[v][len] = sum;
        }
    }

    let mut path = vec![];
    for lvl in 0..=last_layer {
        path.push(queue_by_lvl[lvl][last_path_index[lvl]]);
    }
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut seen = vec![false; n];
    seen[0] = true;
    let mut res = Mod::ZERO;
    let mut path_iter = 0;
    let mut more_layers = last_layer - 1;
    // dbg!(path, more_layers);

    let finish_v = path[path.len() - 2];

    while let Some(v) = queue.pop_front() {
        // dbg!(v);
        res += Mod::ONE;
        if v == finish_v {
            break;
        }
        for &to in &g[v] {
            if !seen[to] {
                seen[to] = true;
                queue.push_back(to);

                if v == path[path_iter] && to == path[path_iter + 1] {
                    path_iter += 1;
                    more_layers -= 1;
                }
            } else {
                // dbg!(v, to, more_layers, dp[to][more_layers - 1]);
                if more_layers > 0 {
                    res += dp[to][more_layers - 1];
                }
            }
        }
        if path_iter == path.len() - 1 {
            break;
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for tc_id in 0..tc {
        dbg!(tc_id, tc);
        let n = input.usize();
        let m = input.usize();
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let fr = input.usize();
            let to = input.usize();
            g[fr].push(to);
            g[to].push(fr);
        }
        for i in 0..n {
            g[i].sort();
        }
        let res = solve_case(&g);
        // let res_stupid = solve_stupid(&g);
        // if res != res_stupid {
        //     dbg!(res, res_stupid);
        //     dbg!(n);
        //     for i in 0..n {
        //         dbg!(i, g[i]);
        //     }
        // }
        // assert_eq!(res, res_stupid);

        out.println(res);
    }
}

fn stress() {
    for it in 40.. {
        dbg!(it);
        let mut rnd = Random::new(123123 + it);
        let n = rnd.gen_range(1..10);
        let m = rnd.gen_range(1..10);
        let mut g = vec![vec![]; n];
        let mut seen = HashSet::new();
        for _ in 0..m {
            let fr = rnd.gen_range(0..n);
            let to = rnd.gen_range(0..n);
            if fr < to {
                if seen.contains(&(fr, to)) {
                    continue;
                }

                seen.insert((fr, to));
                g[fr].push(to);
                g[to].push(fr);
            }
        }
        for i in 0..n {
            g[i].sort();
        }
        let res = solve_case(&g);
        let res_stupid = solve_stupid(&g);
        if res != res_stupid {
            dbg!(res, res_stupid);
            dbg!(n);
            for i in 0..n {
                dbg!(i, g[i]);
            }
            assert!(false);
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
    const PROBLEM_NAME: &str = "f";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    run_dragon(solve, "F", 2..4);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
