use std::assert_ne;
use std::ops::Range;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::misc::rand::Random;
use algo_lib::misc::two_min::TwoMin;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

#[derive(Clone, Copy)]
struct Query {
    segs: [[usize; 2]; 2],
    ci: f64,
}

const INF: f64 = 1e9;

fn inside(seg: [usize; 2], x: usize) -> bool {
    seg[0] <= x && x < seg[1]
}

fn solve_slow(a: &[i32], queries: &[Query]) -> f64 {
    let n = a.len();
    let mut g = Array2D::new(0.0, n, n);
    for i in 0..n {
        for j in 0..n {
            let mx = a[i].max(a[j]);
            let mn = a[i].min(a[j]);
            g[i][j] = (mx as f64) / (mn as f64);
        }
    }
    for query in queries.iter() {
        for i in 0..n {
            for j in 0..n {
                let mut iinside = false;
                iinside |= inside(query.segs[0], i) && inside(query.segs[1], j);
                iinside |= inside(query.segs[1], i) && inside(query.segs[0], j);
                if iinside {
                    g[i][j] *= query.ci;
                }
            }
        }
    }
    let mut edges = vec![];
    for i in 0..n {
        for j in i + 1..n {
            edges.push(Edge {
                from: i,
                to: j,
                cost: g[i][j],
            });
        }
    }
    solve_edges(&mut edges, n)
}

#[derive(Clone, Default)]
struct Node {
    two_min: TwoMin<usize, OEdge>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
struct OEdge {
    cost: OrdF64,
    to: usize,
}

const MAX_EDGE: OEdge = OEdge {
    cost: OrdF64(INF),
    to: usize::MAX,
};

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        let mut res = l.clone();
        res.two_min.merge(&r.two_min);
        res
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        for vals in node.two_min.get_values_mut() {
            vals.1.cost += OrdF64(*update);
        }
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    type Update = f64;

    type Context = ();
}

#[derive(Clone, Debug)]
struct Event {
    pos: usize,
    r: Range<usize>,
    change: f64,
}

fn solve_faster(a: &[i32], queries: &[Query]) -> f64 {
    let n = a.len();

    let mut events = vec![];
    for query in queries.iter() {
        let left = query.segs[0].min(query.segs[1]);
        let right = query.segs[0].max(query.segs[1]);
        let ln = query.ci.ln();
        events.push(Event {
            pos: left[0],
            r: right[0]..right[1],
            change: ln,
        });
        events.push(Event {
            pos: left[1],
            r: right[0]..right[1],
            change: -ln,
        });
        events.push(Event {
            pos: right[0],
            r: left[0]..left[1],
            change: ln,
        });
        events.push(Event {
            pos: right[1],
            r: left[0]..left[1],
            change: -ln,
        });
        {
            let from = left[0].max(right[0]);
            let to = left[1].min(right[1]);
            if from < to {
                events.push(Event {
                    pos: from,
                    r: from..to,
                    change: -ln,
                });
                events.push(Event {
                    pos: to,
                    r: from..to,
                    change: ln,
                });
            }
        }
    }
    events.sort_by_key(|e| e.pos);

    let mut dsu = Dsu::new(n);
    let mut sum_cost = 0.0;
    while dsu.num_components() != 1 {
        let comps: Vec<usize> = (0..n).map(|x| dsu.get(x)).collect();
        let mut st = SegTree::new(n, |pos| {
            let mut two_min = TwoMin::new(usize::MAX, MAX_EDGE);
            let edge = OEdge {
                cost: OrdF64((a[pos] as f64).ln()),
                to: pos,
            };
            two_min.add(comps[pos], edge);
            Node { two_min }
        });
        let mut best_edges: Vec<OEdge> = vec![MAX_EDGE; n];
        let mut ev_it = 0;
        for v in 0..n {
            while ev_it < events.len() && events[ev_it].pos == v {
                let e = &events[ev_it];
                st.update(e.r.clone(), e.change);
                ev_it += 1;
            }

            for seg in [0..v, v + 1..n].iter() {
                if seg.is_empty() {
                    continue;
                }
                if let Some(mut edge) = st.get(seg.clone()).two_min.get_value_by_not_id(comps[v]) {
                    if edge.to > v {
                        edge.cost -= OrdF64((a[v] as f64).ln());
                    } else {
                        edge.cost += OrdF64((a[v] as f64).ln());
                    }
                    assert_ne!(comps[v], comps[edge.to]);
                    best_edges[comps[v]] = best_edges[comps[v]].min(edge);
                }
            }
            st.update(v..v + 1, -2.0 * (a[v] as f64).ln());
        }
        for cid in 0..n {
            let e = best_edges[cid];
            if e.to == usize::MAX {
                continue;
            }
            if dsu.get(cid) != dsu.get(e.to) {
                dsu.unite(cid, e.to);
                sum_cost += e.cost.0.exp();
            }
        }
        if sum_cost > INF {
            return INF;
        }
    }

    sum_cost
}

fn solve_edges(edges: &mut [Edge], n: usize) -> f64 {
    edges.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
    let mut dsu = Dsu::new(n);
    let mut res = 0.0;
    for e in edges.iter() {
        if dsu.get(e.from) != dsu.get(e.to) {
            dsu.unite(e.from, e.to);
            res += e.cost;
        }
    }
    if res > INF || dsu.num_components() > 1 {
        res = INF;
    }
    res
}

#[derive(Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    cost: f64,
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i32>(n);
    let mut queries = vec![];
    for _ in 0..q {
        let mut segs = [[0; 2]; 2];
        for i in 0..2 {
            segs[i][0] = input.usize() - 1;
            segs[i][1] = input.usize();
        }
        let ci = input.f64();
        queries.push(Query { segs, ci });
    }
    let res = solve_faster(&a, &queries);
    out.println(res);
}

fn stress() {
    const MAX_N: usize = 15;
    const MAX_Q: usize = 15;
    const MAX_A: i32 = 15;

    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..MAX_N);
        let q = rnd.gen_range(1..MAX_Q);
        let mut a = rnd.gen_vec(n, 1..MAX_A);
        a.sort();
        for i in 1..n {
            a[i] = a[i].max(a[i - 1] + 1);
        }
        let mut queries = vec![];
        for _ in 0..q {
            let l1 = rnd.gen_range(0..n);
            let r1 = rnd.gen_range(l1 + 1..n + 1);
            let l2 = rnd.gen_range(0..n);
            let r2 = rnd.gen_range(l2 + 1..n + 1);
            let ci = rnd.gen_range(1..10);
            queries.push(Query {
                segs: [[l1, r1], [l2, r2]],
                ci: ci as f64,
            });
        }
        let res_slow = solve_slow(&a, &queries);
        let res_faster = solve_faster(&a, &queries);
        if (res_slow - res_faster).abs() > 1e-3 {
            dbg!(a, res_slow, res_faster);
            for query in queries.iter() {
                dbg!(query.segs, query.ci);
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
    const PROBLEM_NAME: &str = "c_cost_of_conflicts";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "3");
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
