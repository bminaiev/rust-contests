use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
use algo_lib::seg_trees::fenwick::Fenwick;

type Mod = Mod_998_244_353;

fn solve_case(g: &[Vec<usize>]) -> Mod {
    let n = g.len();
    let mut f = Fenwick::<Mod>::new(n + 1);
    let mut biggest_leaf = 0;
    for v in 0..n {
        if g[v].len() == 1 {
            biggest_leaf = v;
        }
    }
    f.add(biggest_leaf, Mod::ONE);
    let mut max_subtree = vec![0; n];
    RecursiveFunction2::new(|f, v: usize, p: usize| {
        for &to in g[v].iter() {
            if to == p {
                continue;
            }
            f.call(to, v);
            max_subtree[v] = max_subtree[v].max(max_subtree[to].max(to));
        }
    })
    .call(n - 1, n - 1);

    for v in 0..n - 1 {
        let my_dp = f.get_range_sum(max_subtree[v] + 1..v);
        // dbg!(v, my_dp);
        f.add(v, my_dp);
        // for prev in max_subtree[v] + 1..v {
        //     let dp_prev = dp[prev];
        //     dp[v] += dp_prev;
        // }
    }
    let mut root_subtrees = vec![];
    for &to in g[n - 1].iter() {
        let cur = to.max(max_subtree[to]);
        root_subtrees.push(cur);
    }
    root_subtrees.sort();
    if root_subtrees.len() <= 1 {
        return Mod::ONE;
    }
    let ok_last_from = root_subtrees[root_subtrees.len() - 2] + 1;
    // dbg!(ok_last_from);
    f.get_range_sum(ok_last_from..n - 1)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    alive_mask: usize,
    res_mask: usize,
}

fn solve_case_slow(g: &[Vec<usize>]) -> Mod {
    let n = g.len();
    let start = State {
        alive_mask: (1 << n) - 1,
        res_mask: 0,
    };
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut seen_res = HashSet::new();
    while let Some(state) = q.pop_front() {
        let mut leafs = vec![];
        for v in 0..n {
            if ((1 << v) & state.alive_mask) != 0 {
                let mut cnt = 0;
                for &to in g[v].iter() {
                    if ((1 << to) & state.alive_mask) != 0 {
                        cnt += 1;
                    }
                }
                if cnt == 1 {
                    leafs.push(v);
                }
            }
        }
        if leafs.len() <= 1 {
            seen_res.insert(state.res_mask);
            continue;
        }
        let max_leaf = leafs[leafs.len() - 1];
        let nres_mask = state.res_mask | (1 << max_leaf);
        for &v in leafs.iter() {
            if v == max_leaf {
                continue;
            }
            let nstate = State {
                alive_mask: state.alive_mask ^ (1 << v),
                res_mask: nres_mask,
            };
            if seen.insert(nstate) {
                q.push_back(nstate);
            }
        }
    }
    Mod::new(seen_res.len() as i32)
}

fn stress() {
    for it in 19.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(2..10);
        let mut g = vec![vec![]; n];
        let mut dsu = Dsu::new(n);
        while dsu.num_components() > 1 {
            let fr = rnd.gen_range(0..n);
            let to = rnd.gen_range(0..n);
            if dsu.get(fr) == dsu.get(to) {
                continue;
            }
            dsu.unite(fr, to);
            g[fr].push(to);
            g[to].push(fr);
        }
        let fast = solve_case(&g);
        let slow = solve_case_slow(&g);
        dbg!(fast, slow);
        if fast != slow {
            dbg!(n);
            // dbg!(g);
            for v in 0..n {
                for &to in g[v].iter() {
                    if to > v {
                        dbg!(v, to);
                    }
                }
            }
            dbg!(fast);
            dbg!(slow);
            assert!(false);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            g[fr].push(to);
            g[to].push(fr);
        }
        out.println(solve_case(&g));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e_";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
