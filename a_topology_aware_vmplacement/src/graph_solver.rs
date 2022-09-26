use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::index_of::IndexOf;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::{output, set_global_output_to_file};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rand::Random;
use algo_lib::{dbg, out, out_line};

use algo_lib::{io::input::Input, misc::gen_vector::gen_vec};

use crate::state::{self, State};
use crate::types::{CreatedVm, TestParams, VmSpec};

fn load_cnt(test_case: usize, params: &TestParams) -> Vec<usize> {
    let mut input = Input::new_file(format!(
        "a_topology_aware_vmplacement/local_test_kit/sample/{:02}-cnt.txt",
        test_case
    ));
    gen_vec(params.vm_specs.len(), |_| input.read())
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
struct GraphNode {
    numa_cnt: usize,
    cpu: u32,
    memory: u32,
}

impl GraphNode {
    pub fn apply(&self, vm_spec: &VmSpec) -> Option<GraphNode> {
        if vm_spec.numa_cnt != self.numa_cnt {
            return None;
        }
        if vm_spec.cpu > self.cpu || vm_spec.memory > self.memory {
            return None;
        }
        Some(GraphNode {
            numa_cnt: self.numa_cnt,
            cpu: self.cpu - vm_spec.cpu,
            memory: self.memory - vm_spec.memory,
        })
    }
}

#[derive(Clone, Debug)]
enum Edge<T> {
    UseVM(T, usize),
    Sum(Vec<T>),
}

impl<T: Clone> Edge<T> {
    fn iter_next_nodes(&self) -> Vec<T> {
        match self {
            Edge::UseVM(node, _) => vec![node.clone()],
            Edge::Sum(vec) => vec.clone(),
        }
    }
}

fn gen_edges(node: &GraphNode, params: &TestParams) -> Vec<Edge<GraphNode>> {
    let mut res = vec![];
    if node.numa_cnt == 4 {
        assert!(params.numa[0] == params.numa[1]);
        assert!(params.numa[2] == params.numa[3]);
        res.push(Edge::Sum(vec![
            GraphNode {
                numa_cnt: 2,
                cpu: params.numa[0].cpu,
                memory: params.numa[0].memory,
            },
            GraphNode {
                numa_cnt: 2,
                cpu: params.numa[2].cpu,
                memory: params.numa[2].memory,
            },
        ]));
        return res;
    }
    assert!(node.numa_cnt <= 2);
    if node.numa_cnt == 2 {
        let one_node = GraphNode {
            numa_cnt: 1,
            cpu: node.cpu,
            memory: node.memory,
        };
        res.push(Edge::Sum(vec![one_node; 2]));
    }
    for i in 0..params.vm_specs.len() {
        if let Some(next) = node.apply(&params.vm_specs[i]) {
            res.push(Edge::UseVM(next, i));
        }
    }
    res
}

fn assert_good_graph(edges: &[Vec<Edge<usize>>]) {
    for i in 0..edges.len() {
        for e in edges[i].iter() {
            match e {
                Edge::UseVM(to, _) => assert!(*to > i),
                Edge::Sum(v) => v.iter().for_each(|to| assert!(*to > i)),
            }
        }
    }
}

struct Graph {
    nodes: Vec<GraphNode>,
    edges: Vec<Vec<Edge<usize>>>,
}

impl Graph {
    pub fn find_node_id(&self, node: &GraphNode) -> usize {
        self.nodes.index_of(node).unwrap()
    }
}

fn gen_graph(params: &TestParams) -> Graph {
    let mut edges = vec![];
    let mut mapping: HashMap<GraphNode, usize> = HashMap::default();
    let mut queue = BTreeSet::new();
    queue.insert(GraphNode {
        numa_cnt: params.numa.len(),
        memory: 0,
        cpu: 0,
    });
    let mut nodes = vec![];
    while !queue.is_empty() {
        let node = queue.last_exn().clone();
        nodes.push(node.clone());
        queue.remove(&node);
        mapping.insert(node.clone(), mapping.len());
        let node_edges = gen_edges(&node, params);
        for e in node_edges.iter() {
            for next_node in e.iter_next_nodes() {
                if !mapping.contains_key(&next_node) {
                    queue.insert(next_node.clone());
                }
            }
        }
        edges.push(node_edges);
    }
    dbg!(edges.len());
    let mut edges_conv = vec![];

    for i in 0..edges.len() {
        let mut node_edges = vec![];
        for e in edges[i].iter() {
            let conv_edge = match e {
                Edge::UseVM(node_to, id) => Edge::UseVM(*mapping.get(node_to).unwrap(), *id),
                Edge::Sum(vec) => {
                    Edge::Sum(vec.iter().map(|node| *mapping.get(node).unwrap()).collect())
                }
            };
            node_edges.push(conv_edge);
        }
        edges_conv.push(node_edges);
    }
    let graph = Graph {
        edges: edges_conv,
        nodes,
    };
    assert_good_graph(&graph.edges);
    graph
}

struct MinimizedGraph {
    mapping: Vec<usize>,
    edges: Vec<Vec<Edge<usize>>>,
}

impl MinimizedGraph {
    // TODO: optimize
    pub fn find_edge_pos(&self, v: usize, vm_id: usize) -> Option<usize> {
        for i in 0..self.edges[v].len() {
            if let Edge::UseVM(_, edge_vm_id) = self.edges[v][i] {
                if edge_vm_id == vm_id {
                    return Some(i);
                }
            }
        }
        None
    }
}

fn minimize_graph(graph: &Graph) -> MinimizedGraph {
    let mut mapping = HashMap::new();
    for i in 0..graph.nodes.len() {
        mapping.insert(i, 0);
    }
    type Mod = Mod_998_244_353;
    let calc_hash = |mapping: &HashMap<usize, usize>, v: usize| -> Mod {
        let mut res = Mod::new(mapping[&v] + 1);
        let mul1 = Mod::new(239017);
        let mul2 = Mod::new(1_000_000_007);
        let mul3 = Mod::new(239);
        for e in graph.edges[v].iter() {
            match e {
                &Edge::UseVM(to_id, vm_id) => {
                    res = res * mul1 + Mod::new(mapping[&to_id]);
                    res = res * mul1 + Mod::new(vm_id);
                    res = res * mul3;
                }
                Edge::Sum(vec) => {
                    for to_id in vec.iter() {
                        res = res * mul1 + Mod::new(mapping[to_id])
                    }
                    res = res * mul2;
                }
            }
        }
        res
    };
    let mut groups = 1;
    loop {
        let mut by_hash = HashMap::new();
        for v in 0..graph.nodes.len() {
            let h = calc_hash(&mapping, v);
            if !by_hash.contains_key(&h) {
                by_hash.insert(h, by_hash.len());
            }
        }
        if by_hash.len() == groups {
            break;
        }
        let mut new_mapping = HashMap::new();
        for v in 0..graph.nodes.len() {
            let h = calc_hash(&mapping, v);
            new_mapping.insert(v, by_hash[&h]);
        }
        mapping = new_mapping;
        groups = by_hash.len();
    }
    dbg!(groups);
    let mut inbound = vec![0; groups];
    let mut example_v = vec![None; groups];
    for v in 0..graph.nodes.len() {
        let id = mapping[&v];
        if example_v[id].is_some() {
            continue;
        }
        example_v[id] = Some(v);
        for e in graph.edges[v].iter() {
            for next_node in e.iter_next_nodes().iter() {
                inbound[mapping[next_node]] += 1;
            }
        }
    }
    let mut queue: VecDeque<_> = (0..inbound.len()).filter(|&v| inbound[v] == 0).collect();
    let mut new_mapping = HashMap::new();
    while let Some(group_id) = queue.pop_front() {
        let v = example_v[group_id].unwrap();
        new_mapping.insert(group_id, new_mapping.len());
        for e in graph.edges[v].iter() {
            for next_node in e.iter_next_nodes().iter() {
                let to = mapping[next_node];
                inbound[to] -= 1;
                if inbound[to] == 0 {
                    queue.push_back(to);
                }
            }
        }
    }
    assert!(new_mapping.len() == groups);

    let mut res_mapping = vec![];
    for v in 0..graph.nodes.len() {
        let to = new_mapping[&mapping[&v]];
        res_mapping.push(to);
    }
    let mut edges = vec![vec![]; groups];
    for v in 0..graph.nodes.len() {
        let new_v = res_mapping[v];
        if edges[new_v].is_empty() && !graph.edges[v].is_empty() {
            for e in graph.edges[v].iter() {
                let new_e = match e {
                    &Edge::UseVM(to_id, vm_id) => Edge::UseVM(res_mapping[to_id], vm_id),
                    Edge::Sum(vec) => Edge::Sum(vec.iter().map(|x| res_mapping[*x]).collect()),
                };
                edges[new_v].push(new_e);
            }
        }
    }
    MinimizedGraph {
        mapping: res_mapping,
        edges,
    }
}

fn normalize(a: &mut [f64]) {
    let sum: f64 = a.iter().sum();
    if sum == 0.0 {
        return;
    }
    for x in a.iter_mut() {
        *x /= sum;
    }
}

fn calc_node_res(
    res: &mut Array2D<f64>,
    edges: &[Vec<Edge<usize>>],
    need_cnt: &[usize],
    probs: &[Vec<f64>],
) {
    for i in 0..res.len() {
        for j in 0..res[i].len() {
            res[i][j] = 0.0;
        }
    }
    for v in (0..edges.len()).rev() {
        for i in 0..edges[v].len() {
            let pr = probs[v][i];
            match &edges[v][i] {
                &Edge::UseVM(to_id, vm_id) => {
                    for j in 0..need_cnt.len() {
                        res[v][j] += pr * res[to_id][j];
                    }
                    res[v][vm_id] += pr;
                }
                Edge::Sum(vec) => {
                    for &to_id in vec.iter() {
                        for j in 0..need_cnt.len() {
                            res[v][j] += pr * res[to_id][j];
                        }
                    }
                }
            }
        }
    }
}

fn calc_scores(res: &Array2D<f64>, need_cnt: &[usize], root: usize) -> Vec<f64> {
    let mut res_pr: Vec<f64> = vec![];
    for j in 0..need_cnt.len() {
        let need = need_cnt[j] as f64 / 4000.0;
        let my = res[root][j];
        let pr = my / need;
        res_pr.push(pr);
    }
    res_pr
}

fn soft_mins(a: &[f64], coefs: &[f64]) -> Vec<f64> {
    let min = min_of_slice(a);
    // TODO: maybe mult by something?
    const C: f64 = 50.0;
    let mut exps: Vec<_> = a
        .iter()
        .zip(coefs)
        .map(|(x, coef)| (C * coef * (min - x)).exp())
        .collect();
    let sum: f64 = exps.iter().sum();
    for x in exps.iter_mut() {
        *x /= sum;
    }
    exps
}

fn min_of_slice(a: &[f64]) -> f64 {
    let mut min = f64::MAX;
    for &x in a.iter() {
        if x < min {
            min = x;
        }
    }
    min
}

struct SolvedGraph {
    alive: Vec<bool>,
    probs: Vec<Vec<f64>>,
}

fn solve_graph(
    edges: &[Vec<Edge<usize>>],
    need_cnt: &[usize],
    root: usize,
    params: &TestParams,
) -> SolvedGraph {
    dbg!(root);

    let coefs: Vec<_> = params
        .vm_specs
        .iter()
        .map(|vm| (vm.numa_cnt as f64) * (vm.cpu + vm.memory) as f64)
        .collect();

    let mut res = Array2D::new(0.0, edges.len(), need_cnt.len());

    let mut global_best = (0.0, 0);
    for glob_iter in 0..1 {
        let mut rnd = Random::new(312353 + glob_iter);

        let mut probs = vec![vec![]; edges.len()];
        for v in 0..probs.len() {
            for _ in 0..edges[v].len() {
                probs[v].push(rnd.gen_double());
            }
            normalize(&mut probs[v]);
        }

        let mut eps = 1e-2;
        let mut prev_score = 0.0;

        for iter in 1..1_001 {
            if eps < 1e-7 {
                break;
            }

            calc_node_res(&mut res, edges, need_cnt, &probs);
            let scores = calc_scores(&res, need_cnt, root);
            // dbg!(scores);
            let cur_score = (min_of_slice(&scores), glob_iter);
            if cur_score > global_best {
                global_best = cur_score;
            }
            const DELTA: f64 = 1e-3;
            if cur_score.0 < prev_score {
                eps *= 1.0 - DELTA;
            } else {
                eps *= 1.0 + DELTA / 10.0;
            }
            prev_score = cur_score.0;
            if iter % 1000 == 0 {
                dbg!(iter, eps, global_best, cur_score);
                // dbg!(scores);

                let mut pr_here = vec![0.0; probs.len()];
                pr_here[0] = 1.0;
                let mut cnt_zero_ver = 0;
                for v in 0..pr_here.len() {
                    if pr_here[v] <= 1e-4 {
                        cnt_zero_ver += 1;
                    }
                    for i in 0..edges[v].len() {
                        match &edges[v][i] {
                            &Edge::UseVM(to_id, _) => pr_here[to_id] += pr_here[v] * probs[v][i],
                            Edge::Sum(vec) => {
                                for &to_id in vec.iter() {
                                    pr_here[to_id] += pr_here[v] * probs[v][i]
                                }
                            }
                        }
                    }
                }

                let mut non_zero = vec![0; params.vm_specs.len() + 1];
                for v in 0..probs.len() {
                    if pr_here[v] < 1e-4 {
                        continue;
                    }
                    let mut cnt_non_zero = 0;
                    for &p in probs[v].iter() {
                        if p > 1e-5 {
                            cnt_non_zero += 1;
                        }
                    }
                    non_zero[cnt_non_zero] += 1;
                }
                dbg!(non_zero);

                // for it in 0..3 {
                //     if pr_here[it] != 0.0 {
                //         dbg!(it, pr_here[it]);
                //         dbg!(edges[it]);
                //         dbg!(probs[it]);
                //         dbg!(res[it]);
                //     }
                // }
                // dbg!(cnt_zero_ver);
            }

            // for v in 0..probs.len() {
            //     let mut any_sum = false;
            //     for e in edges[v].iter() {
            //         if let Edge::Sum(_) = e {
            //             any_sum = true;
            //         }
            //     }
            //     if !any_sum {
            //         for i in 0..res[v].len() {
            //             if let Some(idx) = edges[v].iter().position(|e| {
            //                 if let Edge::UseVM(_, my_vm_id) = e {
            //                     *my_vm_id == i
            //                 } else {
            //                     false
            //                 }
            //             }) {
            //                 probs[v][idx] = res[v][i];
            //             }
            //         }
            //         normalize(&mut probs[v]);
            //     }
            // }

            let soft_scores = soft_mins(&scores, &coefs);
            // dbg!(soft_scores);
            let soft_res: Vec<f64> = (0..res.len())
                .map(|row_id| {
                    res[row_id]
                        .iter()
                        .zip(&soft_scores)
                        .map(|(x, y)| x * y)
                        .sum()
                })
                .collect();
            for v in 0..probs.len() {
                let mut scores_delta = vec![];
                for e in edges[v].iter() {
                    let cur = match e {
                        &Edge::UseVM(v_to, vm_id) => soft_scores[vm_id] + soft_res[v_to],
                        Edge::Sum(vec) => vec.iter().map(|&v_to| soft_res[v_to]).sum(),
                    };
                    scores_delta.push(cur);
                }
                let mut pos_of_max = 0;
                for i in 0..scores_delta.len() {
                    if scores_delta[i] > scores_delta[pos_of_max] {
                        pos_of_max = i;
                    }
                }
                let mut pos_of_min = pos_of_max;
                for i in 0..scores_delta.len() {
                    if probs[v][i] != 0.0 && scores_delta[i] < scores_delta[pos_of_min] {
                        pos_of_min = i;
                    }
                }
                if pos_of_min != pos_of_max {
                    // TODO: think about it.
                    let here_eps = eps; // * (scores_delta[pos_of_max] - scores_delta[pos_of_min]);
                    let change = if probs[v][pos_of_min] < here_eps {
                        probs[v][pos_of_min]
                    } else {
                        here_eps
                    };
                    probs[v][pos_of_min] -= change;
                    probs[v][pos_of_max] += change;
                    // normalize(&mut probs[v]);
                }
            }
        }

        let mut alive = vec![true; probs.len()];

        let mut pr_here = vec![0.0; probs.len()];
        pr_here[0] = 1.0;
        let mut cnt_zero_ver = 0;
        const ALIVE_EPS: f64 = 1e-4;
        const ALIVE_EDGE_EPS: f64 = 1e-2;
        for v in 0..pr_here.len() {
            alive[v] = pr_here[v] > ALIVE_EPS;
            if !alive[v] {
                continue;
            }
            cnt_zero_ver += 1;
            for i in 0..probs[v].len() {
                if probs[v][i] < ALIVE_EDGE_EPS {
                    probs[v][i] = 0.0;
                }
                normalize(&mut probs[v]);
            }
            for i in 0..edges[v].len() {
                match &edges[v][i] {
                    &Edge::UseVM(to_id, _) => pr_here[to_id] += pr_here[v] * probs[v][i],
                    Edge::Sum(vec) => {
                        for &to_id in vec.iter() {
                            pr_here[to_id] += pr_here[v] * probs[v][i]
                        }
                    }
                }
            }
        }
        dbg!(cnt_zero_ver);
        calc_node_res(&mut res, edges, need_cnt, &probs);
        let scores = calc_scores(&res, need_cnt, root);
        // dbg!(scores);
        let cur_score = (min_of_slice(&scores), glob_iter);
        dbg!("after zeroing", cur_score);

        let mut not_used_edges = 0;
        let mut tot_edges = 0;
        for v in 0..pr_here.len() {
            if !alive[v] {
                continue;
            }
            for j in 0..edges[v].len() {
                if let Edge::UseVM(_, vm_id) = edges[v][j] {
                    tot_edges += 1;
                    if res[v][vm_id] <= 1e-3 {
                        not_used_edges += 1;
                    }
                }
            }
        }
        dbg!(not_used_edges, tot_edges);

        return SolvedGraph { alive, probs };
    }
    unreachable!();
}

fn save_graph(solution: &SolvedGraph, minigraph: &MinimizedGraph) {
    set_global_output_to_file("graph.txt");
    out_line!("digraph A {");
    for i in 0..solution.alive.len() {
        if solution.alive[i] {
            out_line!(format!("n{}", i));
        }
    }
    for i in 0..solution.alive.len() {
        if solution.alive[i] {
            for j in 0..solution.probs[i].len() {
                let pr = solution.probs[i][j];
                let e = &minigraph.edges[i][j];
                if pr != 0.0 {
                    if let &Edge::UseVM(to_id, vm_id) = e {
                        if solution.alive[to_id] {
                            out_line!(format!("n{} -> n{}", i, to_id));
                        }
                    }
                }
            }
        }
    }
    out_line!("}");
    output().flush();
}

struct TwoNumaStrategy {
    last_numa2: Vec<usize>,
    // [numa2_idx][graph_node][vm_spec] -> prob
    transitions: Vec<Vec<Vec<f64>>>,
    // [numa2_idx][graph_node] -> prob
    state_probs: Vec<Vec<f64>>,
    root: usize,
    // [numa2_idx][graph_node] -> could <graph_node> in the end become last_numa2[numa2_idx]?
    possible_numa2: Vec<Vec<bool>>,
}

#[derive(Clone, Debug)]
struct TwoNumaState {
    numa2: usize,
    nodes: Vec<[Option<usize>; 2]>,
}

#[derive(Clone, Copy, Debug)]
struct TwoNumaTransition {
    pr: f64,
    pr_of_using_first: f64,
}

impl TwoNumaStrategy {
    pub fn start_state(&self) -> TwoNumaState {
        TwoNumaState {
            numa2: self.root,
            nodes: self
                .last_numa2
                .iter()
                .map(|&node_id| [Some(node_id), Some(node_id)])
                .collect(),
        }
    }

    pub fn apply_transition(
        &self,
        params: &TestParams,
        state: &mut TwoNumaState,
        vm_id: usize,
        numa_id: usize,
        mini_graph: &MinimizedGraph,
    ) {
        if params.vm_specs[vm_id].numa_cnt == 1 {
            let mut exist_possible_state = false;
            for i in 0..state.nodes.len() {
                if let Some(v) = state.nodes[i][numa_id] {
                    if let Some(edge_pos) = mini_graph.find_edge_pos(v, vm_id) {
                        if let Edge::UseVM(to_id, _) = mini_graph.edges[v][edge_pos] {
                            state.nodes[i][numa_id] = Some(to_id);
                            exist_possible_state = true;
                        } else {
                            unreachable!();
                        }
                    } else {
                        state.nodes[i][numa_id] = None;
                    }
                }
            }
            assert!(exist_possible_state);
        } else {
            assert_eq!(params.vm_specs[vm_id].numa_cnt, 2);
            let edge_pos = mini_graph.find_edge_pos(state.numa2, vm_id).unwrap();
            if let Edge::UseVM(to_id, _) = mini_graph.edges[state.numa2][edge_pos] {
                state.numa2 = to_id;
            } else {
                unreachable!();
            }
        }
    }

    pub fn predict_transition(
        &self,
        params: &TestParams,
        state: &TwoNumaState,
        vm_id: usize,
    ) -> Option<TwoNumaTransition> {
        // TODO: maybe different formula?
        let mut numa2_probs = gen_vec(self.last_numa2.len(), |i| {
            if let Some(v1) = state.nodes[i][0] {
                if let Some(v2) = state.nodes[i][1] {
                    // TODO: sqrt seems funny
                    (self.state_probs[i][v1] * self.state_probs[i][v2]).sqrt()
                        * (if self.possible_numa2[i][state.numa2] {
                            1.0
                        } else {
                            0.0
                        })
                } else {
                    0.0
                }
            } else {
                0.0
            }
        });
        normalize(&mut numa2_probs);
        dbg!(numa2_probs);
        if params.vm_specs[vm_id].numa_cnt == 2 {
            let mut pr = 0.0;
            for i in 0..numa2_probs.len() {
                if numa2_probs[i] != 0.0 {
                    pr += self.transitions[i][state.numa2][vm_id] * numa2_probs[i];
                }
            }
            if pr == 0.0 {
                return None;
            }
            return Some(TwoNumaTransition {
                pr,
                pr_of_using_first: 0.0,
            });
        } else {
            assert!(params.vm_specs[vm_id].numa_cnt == 1);
            let mut pr_sum_1 = 0.0;
            let mut pr_sum_2 = 0.0;
            for i in 0..numa2_probs.len() {
                if numa2_probs[i] == 0.0 {
                    continue;
                }
                let pr1 = self.transitions[i][state.nodes[i][0].unwrap()][vm_id] * numa2_probs[i];
                let pr2 = self.transitions[i][state.nodes[i][1].unwrap()][vm_id] * numa2_probs[i];
                pr_sum_1 += pr1;
                pr_sum_2 += pr2;
            }
            if pr_sum_1 + pr_sum_2 == 0.0 {
                return None;
            }
            return Some(TwoNumaTransition {
                pr: (pr_sum_1 + pr_sum_2) / 2.0,
                pr_of_using_first: pr_sum_1 / (pr_sum_1 + pr_sum_2),
            });
        }
    }

    pub fn new(
        solution: &SolvedGraph,
        minigraph: &MinimizedGraph,
        params: &TestParams,
        root: usize,
    ) -> Self {
        let mut rnd = Random::new(455234);
        rnd.gen_u64();

        // TODO: change?
        const MAX_SAMPLES: usize = 100_000;
        // TODO: not used?
        let mut cnt_last_numa2_node = vec![0; minigraph.edges.len()];
        let mut dp_node = vec![];
        let mut last_numa2 = vec![];
        let mut transitions = vec![];
        let mut state_probs = vec![];
        let num_vm_specs = params.vm_specs.len();
        let mut facts = vec![1.0];
        let mut possible_numa2 = vec![];
        for i in 1..50 {
            let next = (i as f64) * facts.last_exn();
            facts.push(next);
        }

        for _ in 0..MAX_SAMPLES {
            let mut v = root;
            let mut used_vms_numa1 = vec![];
            let mut used_vms_numa2 = vec![];
            let mut last_numa2_node = root;

            while !minigraph.edges[v].is_empty() {
                let mut x = rnd.gen_double();
                let mut found_edge = std::usize::MAX;
                for i in 0..solution.probs[v].len() {
                    x -= solution.probs[v][i];
                    if x <= 0.0 {
                        found_edge = i;
                        break;
                    }
                }
                assert!(found_edge < solution.probs[v].len());
                match &minigraph.edges[v][found_edge] {
                    &Edge::UseVM(to_id, vm_spec) => {
                        if params.vm_specs[vm_spec].numa_cnt == 2 {
                            used_vms_numa2.push(vm_spec);
                            // last_numa2_node = to_id;
                        } else {
                            used_vms_numa1.push(vm_spec);
                        }
                        v = to_id
                    }
                    Edge::Sum(vec) => {
                        assert!(vec[0] == vec[1]);
                        v = vec[0]; //rnd.gen(0..vec.len())];
                        last_numa2_node = v;
                    }
                }
            }

            cnt_last_numa2_node[last_numa2_node] += 1;

            let last_numa2_idx = match last_numa2.index_of(&last_numa2_node) {
                Some(x) => x,
                None => {
                    dbg!(last_numa2_node, used_vms_numa2);
                    last_numa2.push(last_numa2_node);
                    transitions.push(vec![vec![0.0; num_vm_specs]; minigraph.edges.len()]);
                    state_probs.push(vec![0.0; minigraph.edges.len()]);
                    possible_numa2.push(vec![false; minigraph.edges.len()]);
                    last_numa2.len() - 1
                }
            };

            for numa_cnt in 1..=2 {
                let used_vms = if numa_cnt == 1 {
                    &used_vms_numa1
                } else {
                    &used_vms_numa2
                };
                let n = used_vms.len();
                dp_node.resize(1 << n, 0);
                dp_node[0] = if numa_cnt == 2 { root } else { last_numa2_node };

                for mask in 0..1 << n {
                    let cur_node = dp_node[mask];
                    if numa_cnt == 2 {
                        possible_numa2[last_numa2_idx][cur_node] = true;
                    }

                    let done = mask.count_ones() as usize;
                    // TODO: is it right?
                    let pr_here = facts[done] * facts[n - done] / facts[n];
                    state_probs[last_numa2_idx][cur_node] += pr_here;

                    let pr = if n == done {
                        0.0
                    } else {
                        facts[done] * facts[n - done - 1] / facts[n]
                    };
                    for next in 0..n {
                        if ((1 << next) & mask) != 0 {
                            continue;
                        }
                        let nmask = mask | (1 << next);
                        let add_vm = used_vms[next];
                        let edge_id = minigraph.find_edge_pos(cur_node, add_vm).unwrap();
                        if let Edge::UseVM(to_node, vm_spec) = minigraph.edges[cur_node][edge_id] {
                            assert!(vm_spec == add_vm);
                            dp_node[nmask] = to_node;
                            transitions[last_numa2_idx][cur_node][vm_spec] += pr;
                        }
                    }
                }
            }
        }

        {
            let mut probs = vec![0.0; last_numa2.len()];
            for v in 0..minigraph.edges.len() {
                if cnt_last_numa2_node[v] != 0 {
                    let idx = last_numa2.index_of(&v).unwrap();
                    probs[idx] += cnt_last_numa2_node[v] as f64;
                }
            }
            normalize(&mut probs);
            dbg!(probs);
        }

        for i in 0..transitions.len() {
            for j in 0..transitions[i].len() {
                normalize(&mut transitions[i][j]);
            }
        }
        dbg!(last_numa2);

        Self {
            state_probs,
            last_numa2,
            transitions,
            root,
            possible_numa2,
        }
    }
}

fn pick_random(solution: &SolvedGraph, minigraph: &MinimizedGraph, params: &TestParams) {
    let mut rnd = Random::new(455234);
    rnd.gen_u64();
    let mut by_len = vec![0; 100];
    let mut cnt_vms_by_type = vec![0; params.vm_specs.len()];

    let mut count: HashMap<Vec<usize>, usize> = HashMap::new();

    for it in 0..1_000_000 {
        let mut v = 0;
        let mut used_vms = vec![];
        while !minigraph.edges[v].is_empty() {
            let mut x = rnd.gen_double();
            let mut found_edge = std::usize::MAX;
            for i in 0..solution.probs[v].len() {
                x -= solution.probs[v][i];
                if x <= 0.0 {
                    found_edge = i;
                    break;
                }
            }
            assert!(found_edge < solution.probs[v].len());
            match &minigraph.edges[v][found_edge] {
                &Edge::UseVM(to_id, vm_spec) => {
                    if params.vm_specs[vm_spec].numa_cnt == 2 {
                        used_vms.push(vm_spec);
                        cnt_vms_by_type[vm_spec] += 1;
                    }
                    v = to_id
                }
                Edge::Sum(vec) => {
                    v = vec[rnd.gen(0..vec.len())];
                }
            }
        }
        by_len[used_vms.len()] += 1;
        used_vms.sort();
        *count.entry(used_vms).or_default() += 1usize;
    }
    for i in 0..by_len.len() {
        if by_len[i] != 0 {
            dbg!(i, by_len[i]);
        }
    }
    dbg!(cnt_vms_by_type);
    for (k, v) in count.iter() {
        dbg!(k, v);
    }
}

#[derive(Clone)]
struct StrategyHandler<'a> {
    strat: &'a TwoNumaStrategy,
    state: TwoNumaState,
    numa_offset: usize,
}

impl<'a> StrategyHandler<'a> {
    pub fn predict_transition(
        &self,
        params: &TestParams,
        vm_id: usize,
    ) -> Option<TwoNumaTransition> {
        self.strat.predict_transition(params, &self.state, vm_id)
    }
}

fn check_baseline_solver(params: &TestParams, vm_ids: &[usize]) {
    let mut machines_stats = params.gen_usage_stats();
    for i in 0..vm_ids.len() {
        let id = vm_ids[i];
        let spec = params.vm_specs[id];
        let mut found = false;
        for m_id in 0..machines_stats.len() {
            let machine = params.get_machine_by_id(m_id);
            if let Some(placement) = machines_stats[m_id].can_place_vm(&spec, machine, 0) {
                machines_stats[m_id].register_vm(&placement);
                found = true;
                break;
            }
        }
        if !found {
            dbg!("baseline score", i, vm_ids.len());
            return;
        }
    }
    dbg!("baseline solved fully!");
}

fn check_solution(
    solution: &SolvedGraph,
    mini_graph: &MinimizedGraph,
    params: &TestParams,
    need_cnt: &[usize],
) {
    dbg!("CHECKING?");
    let strat1 = TwoNumaStrategy::new(&solution, &mini_graph, params, 1);
    let strat2 = TwoNumaStrategy::new(&solution, &mini_graph, params, 2);

    let mut ids = vec![];
    for i in 0..need_cnt.len() {
        ids.extend(vec![i; need_cnt[i]]);
    }
    let mut rnd = Random::new(4342523);
    rnd.shuffle(&mut ids);

    let mut handlers = vec![];
    for (it, &strat) in [&strat1, &strat2].iter().enumerate() {
        handlers.extend(vec![
            StrategyHandler {
                strat,
                state: strat.start_state(),
                numa_offset : it * 2
            };
            5
            // params.total_machines() / 10
        ]);
    }
    {
        let h = &handlers[5];

        let mut sum_p1 = 0.0;
        let mut sum_p2 = 0.0;

        for vm_id in 0..params.vm_specs.len() {
            let p = h.predict_transition(params, vm_id);
            dbg!(vm_id, params.vm_specs[vm_id], p);
            let pr = p.map(|x| x.pr).unwrap_or_default();
            if params.vm_specs[vm_id].numa_cnt == 1 {
                sum_p1 += pr;
            } else {
                sum_p2 += pr;
            }
        }
        dbg!(sum_p1, sum_p2);
        if true {
            return;
        }
    }

    let mut state = State::new(params.clone());

    dbg!(handlers.len());
    dbg!(mini_graph.edges[14]);

    // check_baseline_solver(params, &ids);

    let mut rnd = Random::new(334314);

    for i in 0..ids.len() {
        if i > 0 && (i - 1) % 100 == 0 {
            dbg!(i, ids.len());
        }
        let vm_id = ids[i];
        dbg!(vm_id, params.vm_specs[vm_id]);

        let mut predictions = vec![];
        let mut sum_pr = 0.0;

        for h_id in 0..handlers.len() {
            if let Some(pred) =
                handlers[h_id]
                    .strat
                    .predict_transition(params, &handlers[h_id].state, vm_id)
            {
                sum_pr += pred.pr;
                predictions.push((h_id, pred));
            }
        }
        // dbg!(predictions);
        if predictions.is_empty() {
            dbg!("not found", i);
            break;
        }
        let mut check = rnd.gen_double() * sum_pr;
        let mut use_pred = predictions[0];
        for i in 0..predictions.len() {
            check -= predictions[i].1.pr;
            if check <= 0.0 {
                use_pred = predictions[i];
                break;
            }
        }
        {
            let pred = use_pred.1;
            let h_id = use_pred.0;
            let numa_id = if pred.pr_of_using_first > 0.5 { 0 } else { 1 };

            let mut numa_ids = vec![];
            let numa_offset = handlers[h_id].numa_offset;
            if params.vm_specs[vm_id].numa_cnt == 1 {
                numa_ids.push(numa_id + numa_offset);
            } else {
                numa_ids.push(numa_offset);
                numa_ids.push(numa_offset + 1);
            }
            let new_vm = CreatedVm {
                machine: params.get_machine_by_id(h_id % params.total_machines()),
                numa_ids,
                spec: params.vm_specs[vm_id],
                placement_group_id: 0,
            };
            handlers[h_id].strat.apply_transition(
                params,
                &mut handlers[h_id].state,
                vm_id,
                numa_id,
                mini_graph,
            );
            // dbg!(h_id, new_vm);
            // dbg!(handlers[h_id].state);

            state.register_new_vms(&[new_vm]);
        }
    }
}

pub fn find_shuffling(params: &TestParams, need_cnt: &[usize]) -> bool {
    dbg!("hello");
    let graph = gen_graph(params);
    dbg!("done!");

    let mini_graph = minimize_graph(&graph);
    dbg!(mini_graph.edges.len());
    let debug_node_it = mini_graph.mapping[graph.find_node_id(&GraphNode {
        numa_cnt: 1,
        cpu: 96,
        memory: 64,
    })];
    dbg!(debug_node_it);
    assert_good_graph(&mini_graph.edges);
    let root = mini_graph.mapping[0];
    let solution = solve_graph(&mini_graph.edges, need_cnt, root, params);
    check_solution(&solution, &mini_graph, params, need_cnt);
    // save_graph(&solution, &mini_graph);
    true
}

pub fn find_shuffling_io(test_case: usize, params: &TestParams) -> bool {
    let need_cnt = load_cnt(test_case, params);
    find_shuffling(params, &need_cnt)
}
