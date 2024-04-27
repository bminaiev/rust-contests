//{"name":"A. Multi-Carrier Converged Scheduling","group":"Codeforces - Testing: WF ICPC Challenge, April 2024","url":"https://codeforces.com/gym/478803/problem/A","interactive":false,"timeLimit":4000,"tests":[{"input":"5 7 3\n1 3 10\n3 1 5\n1 5 9\n3 5 11\n2 4 13\n1 2 4\n1 4 7\n","output":"48 2\n3 1 3 5\n2 2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMultiCarrierConvergedScheduling"}}}

use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

struct State {
    in_group: Vec<usize>,
    groups: Vec<Vec<usize>>,
    g: Array2D<i32>,
    score: i32,
}

impl State {
    pub fn new(g: Array2D<i32>) -> Self {
        let n = g.len();
        let mut in_group = vec![0; n];
        let mut groups = vec![vec![]; n];
        for i in 0..n {
            in_group[i] = i;
            groups[i].push(i);
        }
        Self {
            in_group,
            groups,
            g,
            score: 0,
        }
    }

    pub fn move_to_group(&mut self, v: usize, group_id: usize) {
        let old_group = self.in_group[v];
        if old_group == group_id {
            return;
        }
        for &u in &self.groups[old_group] {
            self.score -= self.g[u][v];
        }
        let pos = self.groups[old_group].iter().position(|&x| x == v).unwrap();
        self.groups[old_group].swap_remove(pos);
        self.in_group[v] = group_id;
        self.groups[group_id].push(v);
        for &u in &self.groups[group_id] {
            self.score += self.g[u][v];
        }
    }

    pub fn swap_groups(&mut self, v: usize, u: usize) {
        let old_group_v = self.in_group[v];
        let old_group_u = self.in_group[u];
        self.move_to_group(v, old_group_u);
        self.move_to_group(u, old_group_v);
    }

    pub fn non_empty_groups(&self) -> usize {
        self.groups.iter().filter(|&g| !g.is_empty()).count()
    }

    pub fn get_non_empty_groups(&self) -> Vec<usize> {
        self.groups
            .iter()
            .enumerate()
            .filter(|(_, g)| !g.is_empty())
            .map(|(i, _)| i)
            .collect()
    }
}

const TIME_LIMIT_MS: u128 = 1000;

fn solve_case(g: &Array2D<i32>, max_group_size: usize) -> State {
    let mut start = Instant::now();
    let n = g.len();
    let mut edges = vec![];
    for i in 0..n {
        for j in i + 1..n {
            if g[i][j] == 0 {
                continue;
            }
            edges.push(Edge {
                cost: g[i][j],
                from: i,
                to: j,
            });
        }
    }
    edges.sort();
    edges.reverse();
    let mut state = State::new(g.clone());
    let mut used = vec![false; n];
    for edge in edges.iter() {
        if used[edge.from] || used[edge.to] {
            continue;
        }
        let group_id = state.in_group[edge.from];
        state.move_to_group(edge.to, group_id);
        used[edge.from] = true;
        used[edge.to] = true;
        let mut more_to_join = max_group_size - 2;
        while more_to_join > 0 {
            let mut best = n;
            let mut best_delta_score = 0;
            for i in 0..n {
                if used[i] {
                    continue;
                }
                let cur_score = state.score;
                let old_group = state.in_group[i];
                state.move_to_group(i, group_id);
                let delta_score = state.score - cur_score;
                state.move_to_group(i, old_group);
                if delta_score > best_delta_score {
                    best_delta_score = delta_score;
                    best = i;
                }
            }
            if best == n {
                break;
            }
            state.move_to_group(best, group_id);
            used[best] = true;
            more_to_join -= 1;
        }
    }
    'glob: for _ in 0..5 {
        for edge in edges.iter() {
            if start.elapsed().as_millis() > TIME_LIMIT_MS {
                break 'glob;
            }
            if state.in_group[edge.from] == state.in_group[edge.to] {
                continue;
            }
            let group_id = state.in_group[edge.from];
            for to_swap in state.groups[group_id].clone() {
                let prev_score = state.score;
                state.swap_groups(edge.to, to_swap);
                if state.score < prev_score {
                    state.swap_groups(edge.to, to_swap);
                }
            }
        }
    }
    if max_group_size <= 5 {
        for _ in 0..500 {
            if start.elapsed().as_millis() > TIME_LIMIT_MS * 2 {
                break;
            }
            for edge in edges.iter() {
                let group1 = state.in_group[edge.from];
                let group2 = state.in_group[edge.to];
                if group1 == group2 {
                    continue;
                }
                let mut vertices: Vec<usize> = vec![];
                vertices.extend(&state.groups[group1]);
                vertices.extend(&state.groups[group2]);
                let mut best_score = -1;
                let mut best_mask = 0;
                for mask in 0u32..(1 << vertices.len()) {
                    if mask.count_ones() as usize > max_group_size {
                        continue;
                    }
                    if (vertices.len() - mask.count_ones() as usize) > max_group_size {
                        continue;
                    }
                    let mut score = 0;
                    for i in 0..vertices.len() {
                        for j in i + 1..vertices.len() {
                            let part1 = (mask & (1 << i)) != 0;
                            let part2 = (mask & (1 << j)) != 0;
                            if part1 == part2 {
                                score += g[vertices[i]][vertices[j]];
                            }
                        }
                    }
                    if score > best_score {
                        best_score = score;
                        best_mask = mask;
                    }
                }
                for i in 0..vertices.len() {
                    if (best_mask & (1 << i)) != 0 {
                        state.move_to_group(vertices[i], group1);
                    } else {
                        state.move_to_group(vertices[i], group2);
                    }
                }
            }
        }
    }

    if max_group_size <= 3 {
        let mut rnd = Random::new(787788);
        let mut all_submasks = vec![];
        let max_mask = 1usize << (3 * max_group_size);
        for mask in 0..max_mask {
            if mask.count_ones() as usize == max_group_size {
                all_submasks.push(mask);
            }
        }
        let non_empty_groups = state.get_non_empty_groups();
        while start.elapsed().as_millis() < TIME_LIMIT_MS * 3 {
            let mut groups: Vec<_> = (0..3)
                .map(|_| non_empty_groups[rnd.gen(0..non_empty_groups.len())])
                .collect();
            groups.sort();
            groups.dedup();
            if groups.len() != 3 {
                continue;
            }
            let mut ok = groups
                .iter()
                .all(|&g| state.groups[g].len() == max_group_size);
            let mut vertices: Vec<usize> = vec![];
            for &gr in &groups {
                vertices.extend(&state.groups[gr]);
            }
            for &gr in &groups {
                let mut sum_g = 0;
                for v in &state.groups[gr] {
                    for u in &vertices {
                        if state.in_group[*u] != gr {
                            sum_g += g[*v][*u];
                        }
                    }
                }
                if sum_g == 0 {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
            let mut dp = vec![-1; max_mask];
            let mut dp_prev = vec![0; max_mask];
            dp[0] = 0;
            for mask in 0..max_mask {
                if dp[mask] == -1 {
                    continue;
                }
                for &submask in &all_submasks {
                    if (mask & submask) != 0 {
                        continue;
                    }
                    let mut score = dp[mask];
                    for i in 0..3 * max_group_size {
                        if (submask & (1 << i)) != 0 {
                            for j in i + 1..3 * max_group_size {
                                if (submask & (1 << j)) != 0 {
                                    score += g[vertices[i]][vertices[j]];
                                }
                            }
                        }
                    }
                    let nmask = mask | submask;
                    if dp[nmask] == -1 || dp[nmask] < score {
                        dp[nmask] = score;
                        dp_prev[nmask] = mask;
                    }
                }
            }
            let mut used_submasks = vec![];
            let mut cur_mask = max_mask - 1;
            while cur_mask != 0 {
                used_submasks.push(cur_mask ^ dp_prev[cur_mask]);
                cur_mask = dp_prev[cur_mask];
            }
            for i in 0..3 {
                let mask = used_submasks[i];
                for j in 0..vertices.len() {
                    if (mask & (1 << j)) != 0 {
                        state.move_to_group(vertices[j], groups[i]);
                    }
                }
            }
        }
    }
    state
}

fn stress() {
    let mut rnd = Random::new(787788);
    let n = 100;
    let mut g = Array2D::new_f(n, n, |_, _| rnd.gen(0..10));
    let max_group_size = 3;
    let state = solve_case(&g, max_group_size);
    dbg!(state.score);
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let max_group_size = input.usize();
    let mut g = Array2D::new(0, n, n);
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let cnt = input.i32();
        g[fr][to] += cnt;
        g[to][fr] += cnt;
    }

    let state = solve_case(&g, max_group_size);

    out.println(vec![state.score, state.non_empty_groups() as i32]);
    for group in state.groups.iter() {
        if group.is_empty() {
            continue;
        }
        out.print(group.len() as i32);
        for &v in group.iter() {
            out.print(" ");
            out.print(v + 1);
        }
        out.println("");
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    cost: i32,
    from: usize,
    to: usize,
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_multi_carrier_converged_scheduling";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
