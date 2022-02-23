//{"name":"H. Конструктор","group":"Codeforces - Deltix Round, Summer 2021 (open for everyone, rated, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1556/problem/H?locale=ru","interactive":false,"timeLimit":6000,"tests":[{"input":"10 5\n5 3 4 2 1\n29 49 33 12 55 15 32 62 37\n61 26 15 58 15 22 8 58\n37 16 9 39 20 14 58\n10 15 40 3 19 55\n53 13 37 44 52\n23 59 58 4\n69 80 29\n89 28\n48\n","output":"95\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HKonstruktor"}}}

use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::index_of::IndexOf;
use algo_lib::collections::peek_random::PeekRandom;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::{output, set_global_output_to_file};
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};
// use marathon_utils::dynamic_plot::DynamicPlot;
// use marathon_utils::html_report::HtmlReport;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    cost: i32,
    fr: usize,
    to: usize,
}

fn solve_test(n: usize, mut edges: Vec<Edge>, max_d: &[usize]) -> i32 {
    let mut dsu = Dsu::new(n);
    edges.sort();
    let mut interesting_edges = vec![];
    let checked = max_d.len();
    for edge in edges.iter() {
        if edge.fr < checked || edge.to < checked {
            interesting_edges.push(edge);
        } else if dsu.get(edge.fr) != dsu.get(edge.to) {
            dsu.unite(edge.fr, edge.to);
            interesting_edges.push(edge);
        }
    }
    let mut edges = interesting_edges;
    let mut res = i32::MAX;
    let mut additional_cost = vec![0; n];
    let mut rnd = Random::new(787788);
    let start = Instant::now();
    let mut free_deg = vec![0; n];
    while start.elapsed().as_millis() < 60 {
        let max_add = rnd.gen_in_range(1..100);
        for i in 0..checked {
            additional_cost[i] = rnd.gen_in_range(0..max_add);
            free_deg[i] = max_d[i];
        }
        for i in checked..n {
            free_deg[i] = n;
        }
        edges.sort_by_key(|edge| edge.cost + additional_cost[edge.fr] + additional_cost[edge.to]);
        dsu.clear();
        let mut cur_cost = 0;
        for edge in edges.iter() {
            if dsu.get(edge.fr) != dsu.get(edge.to)
                && free_deg[edge.fr] > 0
                && free_deg[edge.to] > 0
            {
                free_deg[edge.fr] -= 1;
                free_deg[edge.to] -= 1;
                dsu.unite(edge.fr, edge.to);
                cur_cost += edge.cost;
            }
        }
        if dsu.num_components() != 1 {
            cur_cost += (dsu.num_components() as i32) * 100_000;
        }
        res.update_min(cur_cost);
    }
    res
}

struct Scorer {
    cur_cost: i32,
    used_edges: Vec<Edge>,
    free_deg: Vec<usize>,
    dsu: Dsu,
}

impl Scorer {
    pub fn new(free_deg: Vec<usize>) -> Self {
        let dsu = Dsu::new(free_deg.len());
        Self {
            cur_cost: 0,
            used_edges: vec![],
            free_deg,
            dsu,
        }
    }

    pub fn remove_edge(&mut self, edge: Edge) {
        let idx = self.used_edges.index_of(&edge).unwrap();
        self.free_deg[edge.fr] += 1;
        self.free_deg[edge.to] += 1;
        self.cur_cost -= edge.cost;
        self.used_edges.swap_remove(idx);
        self.dsu.clear();
        for e in self.used_edges.iter() {
            self.dsu.unite(e.fr, e.to);
        }
    }

    pub fn remove_random_edge(&mut self, rnd: &mut Random) -> Edge {
        let idx = rnd.gen_in_range(0..self.used_edges.len());
        let edge = self.used_edges[idx];
        self.remove_edge(edge);
        edge
    }

    pub fn add_edge(&mut self, e: Edge) -> bool {
        if self.dsu.get(e.fr) == self.dsu.get(e.to)
            || self.free_deg[e.fr] == 0
            || self.free_deg[e.to] == 0
        {
            false
        } else {
            self.free_deg[e.fr] -= 1;
            self.free_deg[e.to] -= 1;
            self.cur_cost += e.cost;
            self.used_edges.push(e);
            self.dsu.unite(e.fr, e.to);
            true
        }
    }
}

fn solve_sa(n: usize, mut edges: Vec<Edge>, max_d: &[usize], max_time: f64) -> i32 {
    let mut dsu = Dsu::new(n);
    edges.sort();
    let mut interesting_edges = vec![];
    let checked = max_d.len();
    for edge in edges.iter() {
        if edge.fr < checked || edge.to < checked || edge.to == n - 1 || true {
            interesting_edges.push(edge);
        } else if dsu.get(edge.fr) != dsu.get(edge.to) {
            dsu.unite(edge.fr, edge.to);
            interesting_edges.push(edge);
        }
    }
    let edges = interesting_edges;
    // assert!(interesting_edges.len() <= n * (max_d.len() + 1));
    let mut res = i32::MAX;
    let mut rnd = Random::new(787788);
    let mut free_deg = vec![n; n];
    for i in 0..checked {
        free_deg[i] = max_d[i];
    }
    let mut scorer = Scorer::new(free_deg);
    for &e in edges.iter() {
        if e.to == n - 1 || e.fr == n - 1 {
            assert!(scorer.add_edge(*e));
        }
    }
    assert_eq!(scorer.dsu.num_components(), 1);
    // let mut html = HtmlReport::new("outputs".to_owned(), "index");
    // let mut plot = DynamicPlot::new("score", "time (ms)", "score");
    let mut sa = SimulatedAnnealing::new(max_time, SearchFor::MinimumScore, 100.0, 0.1);
    while sa.should_continue() {
        // plot.add_point(sa.elapsed_ms(), scorer.cur_cost);
        res.update_min(scorer.cur_cost);
        let prev_score = scorer.cur_cost;
        let e_to_remove = scorer.remove_random_edge(&mut rnd);
        loop {
            let e_to_add = **edges.peek_random(&mut rnd).unwrap();
            if scorer.add_edge(e_to_add) {
                if !sa.should_go(prev_score, scorer.cur_cost) {
                    scorer.remove_edge(e_to_add);
                    assert!(scorer.add_edge(e_to_remove));
                }
                break;
            }
        }
        assert_eq!(scorer.dsu.num_components(), 1);
    }
    // html.add_dynamic_plot(plot);
    // html.save().unwrap();
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let max_d = input.vec::<usize>(k);
    let mut edges = vec![];
    for i in 0..n {
        for j in i + 1..n {
            let cost = input.i32();
            edges.push(Edge { fr: i, to: j, cost });
        }
    }
    let res = solve_test(n, edges.clone(), &max_d);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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

/*
fn stress() {
    for it in 82.. {
        dbg!(it);
        let mut rnd = Random::new(7 + it);
        let n = 50; //rnd.gen_in_range(2..50);
        let mut edges = vec![];
        for i in 0..n {
            for j in i + 1..n {
                edges.push(Edge {
                    fr: i,
                    to: j,
                    cost: rnd.gen_in_range(1..100),
                });
            }
        }
        let d_len = 5; //rnd.gen_in_range(1..min(n, 6));
        let max_d = rnd.gen_vec(d_len, 1..n);
        let res = solve_test(n, edges.clone(), &max_d);
        let res_sa = solve_sa(n, edges, &max_d);
        assert_eq!(res_sa, res);
    }
}

*/

#[allow(unused)]
fn stress2() {
    for it in 51331.. {
        dbg!(it);
        let mut rnd = Random::new(7 + it);
        let n = 50; //rnd.gen_in_range(2..50);

        let mut d = Array2D::new(100, n, n);
        let mx = rnd.gen_in_range(2..80);
        dbg!(mx);
        for i in 0..n {
            for j in i + 1..n {
                d[i][j] = rnd.gen_in_range(1..mx);
            }
        }
        let max_d = gen_vec(N, |_| rnd.gen_in_range(1..20));
        let mx_balance = rnd.gen_in_range(5..20);
        let balance = gen_vec(n, |_| rnd.gen_in_range(3..mx_balance));
        dbg!(balance);
        const N: usize = 5;
        for x in 0..N {
            for y in x + 1..N {
                d[x][y] = rnd.gen_in_range(1..mx);
            }
            for y in N..n {
                let add = if rnd.gen_in_range(0..N / max_d[x] + 1) == 0 {
                    0
                } else {
                    1
                };
                d[x][y] = balance[y] + add;
            }
        }

        set_global_output_to_file("test.txt");
        out_line!(n, N);
        let mut edges = vec![];
        for i in 0..n {
            for j in i + 1..n {
                edges.push(Edge {
                    fr: i,
                    to: j,
                    cost: d[i][j],
                });
            }
        }
        for e in edges.iter() {
            d[e.fr][e.to] = e.cost;
            d[e.to][e.fr] = e.cost;
            assert!(e.cost >= 1);
            assert!(e.cost <= 100);
        }
        out_line!(max_d);
        for i in 0..n - 1 {
            for j in i + 1..n {
                out!(d[i][j]);
                if j != n - 1 {
                    out!(" ");
                }
            }
            out_line!();
        }
        output().flush();
        let res = solve_test(n, edges.clone(), &max_d);
        let mut ok = false;
        for &time in [0.1, 0.3, 1.0, 5.0, 10.0].iter() {
            dbg!(time);
            let res_sa = solve_sa(n, edges.clone(), &max_d, time);
            if res_sa == res {
                ok = true;
                break;
            }
        }
        assert!(ok);
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // stress2();
}
//END MAIN
