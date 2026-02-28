//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::{BTreeMap, BTreeSet};
use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::trees::centroid_decomposition::CentroidDecomposition;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::two_min::TwoMin;

fn conv_colors(c: u8) -> usize {
    match c {
        b'R' => 0,
        b'G' => 1,
        b'Y' => 2,
        _ => unreachable!(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Debug)]
struct Idx {
    dist: usize,
    repr: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Value {
    dist: usize,
    v: usize,
}

#[derive(Clone, Debug)]
struct Paths {
    for_repr: BTreeMap<usize, BTreeSet<Value>>,
    all_repr: BTreeSet<Idx>,
}

impl Paths {
    fn new() -> Self {
        Self {
            for_repr: BTreeMap::new(),
            all_repr: BTreeSet::new(),
        }
    }

    fn add(&mut self, idx: Idx, v: usize) {
        let repr = idx.repr;
        if let Some(set) = self.for_repr.get(&repr) {
            let best = set.iter().next().unwrap();
            self.all_repr.remove(&Idx {
                dist: best.dist,
                repr,
            });
        }
        self.for_repr
            .entry(repr)
            .or_default()
            .insert(Value { dist: idx.dist, v });
        let best = self.for_repr[&repr].iter().next().unwrap();
        self.all_repr.insert(Idx {
            dist: best.dist,
            repr,
        });
    }

    fn remove(&mut self, idx: Idx, v: usize) {
        let repr = idx.repr;
        let set = self.for_repr.get_mut(&repr).unwrap();
        set.remove(&Value { dist: idx.dist, v });

        self.all_repr.remove(&Idx {
            dist: idx.dist,
            repr,
        });
        if let Some(best) = set.iter().next() {
            self.all_repr.insert(Idx {
                dist: best.dist,
                repr,
            });
        } else {
            self.for_repr.remove(&repr);
        }
    }

    fn get_two_min(&self) -> TwoMin<usize, usize> {
        let mut res = TwoMin::new(usize::MAX, usize::MAX / 3);
        for idx in self.all_repr.iter().take(2) {
            res.add(idx.repr, idx.dist);
        }
        res
    }
}

struct Solver {
    centroid: CentroidDecomposition,
    paths: Vec<Vec<Paths>>,
    color: Vec<usize>,
}

impl Solver {
    fn new(g: &[Vec<usize>], color: &[usize]) -> Self {
        let mut centroid = CentroidDecomposition::new(g);
        // let mut g = g.to_vec();
        // for v in 0..g.len() {
        //     g[v].sort();
        // }
        // for v in 0..g.len() {
        //     for remote in centroid.ups[v].iter_mut() {
        //         if remote.to == v {
        //             continue;
        //         }
        //         // remote.last_on_path = g[remote.to].binary_search(&remote.last_on_path).unwrap();
        //         // remote.first_on_path = g[v].binary_search(&remote.first_on_path).unwrap();
        //     }
        // }
        let mut paths = vec![vec![Paths::new(); g.len()]; 2];
        for v in 0..g.len() {
            let my_color = color[v];
            if color[v] == 2 {
                continue;
            }
            for remote in &centroid.ups[v] {
                // dbg!(v, remote);
                paths[my_color][remote.to].add(
                    Idx {
                        dist: remote.dist as usize,
                        repr: remote.last_on_path,
                    },
                    v,
                );
            }
        }
        // dbg!(paths[0][1]);
        Self {
            centroid,
            paths,
            color: color.to_vec(),
        }
    }

    fn update_color(&mut self, v: usize, c: usize) {
        if self.color[v] != 2 {
            for remote in &self.centroid.ups[v] {
                self.paths[self.color[v]][remote.to].remove(
                    Idx {
                        dist: remote.dist as usize,
                        repr: remote.last_on_path,
                    },
                    v,
                );
            }
        }
        self.color[v] = c;
        if self.color[v] != 2 {
            for remote in &self.centroid.ups[v] {
                self.paths[self.color[v]][remote.to].add(
                    Idx {
                        dist: remote.dist as usize,
                        repr: remote.last_on_path,
                    },
                    v,
                );
            }
        }
    }

    fn query(&self, v: usize) -> i64 {
        let mut mins = [
            self.paths[0][v].get_two_min(),
            self.paths[1][v].get_two_min(),
        ];
        for remote in &self.centroid.ups[v] {
            if remote.to == v {
                continue;
            }
            // dbg!("checking", remote);
            for color in 0..2 {
                let remote_two_mins = self.paths[color][remote.to].get_two_min();
                if let Some(dist) = remote_two_mins.get_value_by_not_id(remote.last_on_path) {
                    mins[color].add(remote.first_on_path, dist + remote.dist as usize);
                }
            }
        }
        let mut res = usize::MAX;
        // dbg!(mins[0].get_values());
        // dbg!(mins[1].get_values());
        for min0 in mins[0].get_values() {
            for min1 in mins[1].get_values() {
                if min0.0 != min1.0 {
                    res = res.min(min0.1 + min1.1);
                }
            }
        }
        if res >= usize::MAX / 5 {
            -1
        } else {
            res as i64
        }
    }
}

struct SolverSimple {
    g: Vec<Vec<usize>>,
    color: Vec<usize>,
}

impl SolverSimple {
    fn new(g: &[Vec<usize>], color: &[usize]) -> Self {
        Self {
            g: g.to_vec(),
            color: color.to_vec(),
        }
    }

    fn update_color(&mut self, v: usize, c: usize) {
        self.color[v] = c;
    }

    fn query(&self, v: usize) -> i64 {
        assert_eq!(self.color[v], 2);
        let mut res = i64::MAX;
        for start in 0..self.g.len() {
            if self.color[start] == 0 {
                let dist = self.dfs(start, start, v, false);
                res = res.min(dist);
            }
        }
        if res >= i64::MAX / 10 {
            res = -1;
        }
        res
    }

    fn dfs(&self, v: usize, p: usize, mid: usize, seen_mid: bool) -> i64 {
        let mut res = i64::MAX / 2;
        if seen_mid && self.color[v] == 1 {
            return 0;
        }
        for &to in &self.g[v] {
            if to != p {
                res = res.min(self.dfs(to, v, mid, seen_mid || to == mid) + 1);
            }
        }
        res
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let q = input.usize();
        let mut g = vec![vec![]; n];
        for _ in 0..(n - 1) {
            let u = input.usize() - 1;
            let v = input.usize() - 1;
            g[u].push(v);
            g[v].push(u);
        }
        let start_colors = input.string();
        let mut color = vec![0; n];
        for i in 0..n {
            color[i] = conv_colors(start_colors[i]);
        }
        let mut solver = Solver::new(&g, &color);
        for _ in 0..q {
            let q_type = input.usize();
            if q_type == 1 {
                let v = input.usize() - 1;
                let c = conv_colors(input.string()[0]);
                solver.update_color(v, c);
            } else {
                assert_eq!(q_type, 2);
                let v = input.usize() - 1;
                let res = solver.query(v);
                out.println(res);
            }
        }
    }
}

fn stress() {
    for it in 13470.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..129);
        let mut g = vec![vec![]; n];
        for i in 1..n {
            let v = rnd.gen_range(0..i);
            g[v].push(i);
            g[i].push(v);
        }
        let mut color = vec![0; n];
        for i in 0..n {
            color[i] = rnd.gen_range(0..3);
        }
        // dbg!(&g, &color);
        let mut solver = Solver::new(&g, &color);
        let mut solver_simple = SolverSimple::new(&g, &color);
        for _ in 0..100 {
            let v = rnd.gen_range(0..n);
            if rnd.gen_bool() && color[v] == 2 {
                let res = solver.query(v);
                let res_simple = solver_simple.query(v);
                // dbg!(v, res, res_simple);
                assert_eq!(res, res_simple);
            } else {
                let c = rnd.gen_range(0..3);
                solver.update_color(v, c);
                solver_simple.update_color(v, c);
                color[v] = c;
            }
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
    const PROBLEM_NAME: &str = "g";
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
