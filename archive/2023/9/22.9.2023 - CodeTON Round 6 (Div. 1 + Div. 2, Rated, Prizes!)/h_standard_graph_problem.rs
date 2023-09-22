//{"name":"H. Standard Graph Problem","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5 6\n1 2 1\n2 3 5\n3 2 3\n4 1 8\n2 1 4\n+ 1\n- 1\n+ 3\n+ 1\n+ 4\n+ 2\n","output":"15\n-1\n14\n12\n4\n0\n"},{"input":"10 14 10\n8 6 4\n2 5 1\n3 5 4\n1 6 3\n1 3 7\n7 2 1\n6 1 3\n4 10 1\n4 6 5\n5 4 1\n5 8 10\n10 9 1\n9 5 1\n9 7 6\n+ 7\n+ 8\n- 7\n+ 10\n+ 2\n- 10\n+ 5\n- 2\n- 5\n+ 3\n","output":"28\n24\n29\n19\n18\n24\n18\n19\n29\n20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HStandardGraphProblem"}}}

use std::cmp::min;
use std::collections::BTreeSet;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::hld::Hld;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    cost: i64,
    fr: usize,
    to: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    cost: i64,
    v: usize,
}

#[derive(Clone, Copy, Debug)]
struct ParentEdge {
    cost: i64,
    to: usize,
}

#[derive(Clone, Copy, Default)]
struct Node {
    min_marked: i64,
    sum_min_marked: i64,
}

impl Node {
    fn get_sum(&self) -> i64 {
        if self.min_marked == 0 {
            self.sum_min_marked
        } else {
            0
        }
    }
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        let min_marked = min(l.min_marked, r.min_marked);
        let mut sum = 0;
        if l.min_marked == min_marked {
            sum += l.sum_min_marked;
        }
        if r.min_marked == min_marked {
            sum += r.sum_min_marked;
        }
        Self {
            min_marked,
            sum_min_marked: sum,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.min_marked += update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    type Update = i64;

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let q = input.usize();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let cost = input.i64();
        g[fr].push(Edge { fr, to, cost })
    }
    const INF: i64 = 1e12 as i64;
    for fr in 0..n {
        let to = (fr + 1) % n;
        g[fr].push(Edge { fr, to, cost: INF })
    }

    let mut edges_pq = vec![BTreeSet::<Edge>::new(); n];
    let mut edges_pq_cost_delta = vec![0; n];
    for v in 0..n {
        for e in g[v].iter() {
            edges_pq[v].insert(*e);
        }
    }
    let mut parent: Vec<Option<ParentEdge>> = vec![None; n];
    let mut cur_cost = vec![0; n];
    let mut path_comps = vec![0];
    let mut path_comps_edges = vec![];

    let mut dsu = Dsu::new(n);
    let mut dsu_comp = gen_vec(n, |x| x);

    let mut seen = vec![false; n];
    seen[0] = true;

    loop {
        let last_comp_id = path_comps[path_comps.len() - 1];
        let best_edge: Option<Edge> = edges_pq[last_comp_id].first().copied();
        if let Some(mut best_edge) = best_edge {
            edges_pq[last_comp_id].remove(&best_edge);
            best_edge.cost += edges_pq_cost_delta[last_comp_id];
            let to = best_edge.to;
            if dsu.get(to) == dsu.get(best_edge.fr) {
                continue;
            }
            if seen[to] {
                // compact
                let new_comp_id = parent.len();
                let first_comp_id = dsu_comp[dsu.get(to)];
                parent.push(None);
                cur_cost.push(0);
                edges_pq.push(BTreeSet::new());
                edges_pq_cost_delta.push(0);
                path_comps_edges.push(best_edge);

                while let Some(last_comp) = path_comps.pop() {
                    let used_edge: Edge = path_comps_edges.pop().unwrap();
                    dsu.unite(used_edge.fr, used_edge.to);
                    dsu_comp[dsu.get(used_edge.fr)] = new_comp_id;

                    parent[last_comp] = Some(ParentEdge {
                        cost: used_edge.cost,
                        to: new_comp_id,
                    });
                    edges_pq_cost_delta[last_comp] -= used_edge.cost;
                    if edges_pq[last_comp].len() > edges_pq[new_comp_id].len() {
                        edges_pq.swap(last_comp, new_comp_id);
                        edges_pq_cost_delta.swap(last_comp, new_comp_id);
                    }
                    for e in edges_pq[last_comp].clone() {
                        edges_pq[new_comp_id].insert(Edge {
                            fr: e.fr,
                            to: e.to,
                            cost: e.cost + edges_pq_cost_delta[last_comp]
                                - edges_pq_cost_delta[new_comp_id],
                        });
                    }

                    cur_cost[used_edge.fr] = used_edge.cost;

                    if last_comp == first_comp_id {
                        break;
                    }
                }
                path_comps.push(new_comp_id);
            } else {
                path_comps.push(to);
                path_comps_edges.push(best_edge);
                seen[to] = true;
            }
        } else {
            break;
        }
    }
    assert!(path_comps.len() == 1);
    let tree_root = path_comps[0];
    assert!(tree_root == parent.len() - 1);
    parent[tree_root] = Some(ParentEdge {
        cost: INF,
        to: tree_root,
    });
    let mut sum_costs = 0;
    for node in 0..parent.len() {
        let parent_edge = parent[node].unwrap();
        sum_costs += parent_edge.cost;
    }

    let mut g = vec![vec![]; parent.len()];
    for v in 0..parent.len() {
        let p = parent[v].unwrap().to;
        if p != v {
            g[p].push(v);
        }
    }
    let n = g.len();
    let hld = Hld::new(g, tree_root);
    let mut st = SegTree::new(n, |pos| Node {
        min_marked: 0,
        sum_min_marked: parent[hld.order[pos]].unwrap().cost,
    });

    for _ in 0..q {
        let q_type = input.string_as_string();
        let delta = if q_type == "+" {
            1
        } else {
            assert_eq!(q_type, "-");
            -1
        };
        let v = input.usize() - 1;
        for seg in hld.find_path_segs(v, tree_root) {
            sum_costs -= st.get(seg.clone()).get_sum();
            st.update(seg.clone(), delta);
            sum_costs += st.get(seg.clone()).get_sum();
        }
        if sum_costs >= INF / 2 {
            out_line!(-1);
        } else {
            out_line!(sum_costs);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
