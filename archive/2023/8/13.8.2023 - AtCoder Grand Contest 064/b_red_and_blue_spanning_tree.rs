//{"name":"B - Red and Blue Spanning Tree","group":"AtCoder - AtCoder Grand Contest 064","url":"https://atcoder.jp/contests/agc064/tasks/agc064_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2 R\n1 3 B\n2 3 B\nRRB\n","output":"Yes\n2 1\n"},{"input":"3 4\n1 2 R\n1 2 B\n1 3 B\n2 3 B\nRRR\n","output":"No\n"},{"input":"8 16\n5 7 B\n2 7 R\n1 6 R\n1 4 R\n6 7 R\n4 6 B\n4 8 R\n2 3 R\n3 5 R\n6 7 B\n2 6 B\n5 6 R\n1 3 B\n4 5 B\n2 7 B\n1 8 B\nBRBRRBRB\n","output":"Yes\n1 2 4 9 11 13 16\n"},{"input":"8 10\n1 7 R\n1 3 B\n2 5 B\n2 8 R\n1 5 R\n3 6 R\n2 6 B\n3 4 B\n2 8 B\n4 6 B\nRRRBBBRB\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRedAndBlueSpanningTree"}}}

use std::collections::VecDeque;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Edge {
    fr: usize,
    to: usize,
    is_red: bool,
    id: usize,
}

fn solve_one(edges: &mut Vec<Edge>, need_red: &[bool]) -> Option<Vec<usize>> {
    let n = need_red.len();
    let mut rnd = Random::new(787788);
    rnd.shuffle(edges);
    let mut g = vec![vec![]; n];
    for i in 0..edges.len() {
        g[edges[i].fr].push(i);
        g[edges[i].to].push(i);
    }
    let mut dsu = Dsu::new(n);
    let mut res = vec![];
    let mut ok = vec![false; n];
    for e in edges.iter() {
        if dsu.get(e.fr) != dsu.get(e.to) {
            if e.is_red == need_red[e.fr] && e.is_red == need_red[e.to] && !ok[e.fr] && !ok[e.to] {
                dsu.unite(e.fr, e.to);
                res.push(e.id);
                ok[e.fr] = true;
                ok[e.to] = true;
            }
        }
    }
    let mut to_check = VecDeque::new();
    for i in 0..edges.len() {
        to_check.push_back(i);
    }
    while let Some(e_id) = to_check.pop_front() {
        let e = edges[e_id];
        for (fr, to) in [(e.fr, e.to), (e.to, e.fr)].into_iter() {
            if dsu.get(fr) != dsu.get(to) {
                if e.is_red == need_red[fr] && ok[to] {
                    dsu.unite(e.fr, e.to);
                    res.push(e.id);
                    ok[fr] = true;
                    for &e_id in g[fr].iter() {
                        to_check.push_back(e_id);
                    }
                }
            }
        }
    }
    for e in edges.iter() {
        if dsu.get(e.fr) != dsu.get(e.to) {
            dsu.unite(e.fr, e.to);
            res.push(e.id);
        }
    }
    if ok.iter().all(|x| *x) {
        Some(res)
    } else {
        None
    }
}

fn solve(input: &mut Input) {
    let _n = input.usize();
    let m = input.usize();
    let mut edges = gen_vec(m, |id| {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let is_red = input.string_as_string() == "R";
        Edge { fr, to, is_red, id }
    });
    let s = input.string();
    let need_red: Vec<_> = s.iter().map(|c| *c == b'R').collect();
    for _it in 0..10 {
        if let Some(res) = solve_one(&mut edges, &need_red) {
            out_line!("Yes");
            for r in res {
                out!(r + 1, "");
            }
            out_line!();
            return;
        }
    }
    out_line!("No");
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
