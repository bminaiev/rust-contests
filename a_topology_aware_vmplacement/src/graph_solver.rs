use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::math::frac::Frac;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
use algo_lib::{dbg, out, out_line};

use algo_lib::{io::input::Input, misc::gen_vector::gen_vec};

use crate::types::{TestParams, VmSpec};
use crate::usage_stats::MachineUsedStats;
use crate::usage_stats::NumaUsedStats;

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

fn solve_graph(edges: &[Vec<Edge<usize>>], need_cnt: &[usize], root: usize) {
    dbg!(root);
    let mut rnd = Random::new(341425);
    let mut probs = vec![vec![]; edges.len()];
    for v in 0..probs.len() {
        for _ in 0..edges[v].len() {
            probs[v].push(rnd.gen_double());
        }
        normalize(&mut probs[v]);
    }
    let mut res = vec![vec![0.0; need_cnt.len()]; edges.len()];
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
    for j in 0..need_cnt.len() {
        let need = need_cnt[j] as f64 / 4000.0;
        let my = res[root][j];
        let pr = my / need;
        dbg!(j, pr);
    }
}

pub fn find_shuffling(params: &TestParams, need_cnt: &[usize]) -> bool {
    dbg!("hello");
    let graph = gen_graph(params);
    dbg!("done!");
    let mini_graph = minimize_graph(&graph);
    dbg!(mini_graph.edges.len());
    assert_good_graph(&mini_graph.edges);
    let root = mini_graph.mapping[0];
    solve_graph(&mini_graph.edges, need_cnt, root);
    true
}

pub fn find_shuffling_io(test_case: usize, params: &TestParams) -> bool {
    let mut need_cnt = load_cnt(test_case, params);
    find_shuffling(params, &need_cnt)
}
