use std::collections::BTreeSet;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

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

fn nearby(seg: [usize; 2], n: usize) -> Vec<usize> {
    let mut res = vec![];
    if seg[0] > 0 {
        res.push(seg[0] - 1);
    }
    res.push(seg[0]);
    // for x in seg[0]..seg[1] {
    //     res.push(x);
    // }
    res.push(seg[1] - 1);
    if seg[1] < n {
        res.push(seg[1]);
    }
    res
}

fn solve_faster(a: &[i32], queries: &[Query]) -> f64 {
    let n = a.len();
    let mut g = Array2D::new(0.0, n, n);
    for i in 0..n {
        for j in 0..n {
            let mx = a[i].max(a[j]);
            let mn = a[i].min(a[j]);
            g[i][j] = (mx as f64) / (mn as f64);
        }
    }
    let mut edges = BTreeSet::new();
    for i in 0..n - 1 {
        edges.insert((i, i + 1));
    }
    for query in queries.iter() {
        let n1 = nearby(query.segs[0], n);
        let n2 = nearby(query.segs[1], n);
        for &v1 in n1.iter() {
            for &v2 in n2.iter() {
                let mn = v1.min(v2);
                let mx = v1.max(v2);
                edges.insert((mn, mx));
            }
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
    let mut edges_vec = vec![];
    for (fr, to) in edges.iter() {
        edges_vec.push(Edge {
            from: *fr,
            to: *to,
            cost: g[*fr][*to],
        });
    }
    solve_edges(&mut edges_vec, n)
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
    let res = solve_slow(&a, &queries);
    out.println(res);
}

fn stress() {
    const MAX_N: usize = 5;
    const MAX_Q: usize = 5;
    const MAX_A: i32 = 5;

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
        if (res_slow - res_faster).abs() > 1e-6 {
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
    const PROBLEM_NAME: &str = "c";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
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
