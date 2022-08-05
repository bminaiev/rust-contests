//{"name":"Wonderland Chase","group":"Google Coding Competitions - World Finals 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/000000000087762e/0000000000b9c499","interactive":false,"timeLimit":10000,"tests":[{"input":"4\n5 5 5 1\n1 2\n1 3\n2 4\n3 4\n4 5\n5 5 5 2\n1 2\n1 3\n2 4\n3 4\n4 5\n3 1 2 3\n1 3\n2 1 1 2\n1 2\n","output":"Case #1: SAFE\nCase #2: 4\nCase #3: SAFE\nCase #4: 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WonderlandChase"}}}

use std::collections::VecDeque;

use algo_lib::graph::bfs::bfs;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Edge {
    id: usize,
    to: usize,
}

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut g = vec![vec![]; n];
    let alice_start = input.usize() - 1;
    let queen_start = input.usize() - 1;
    for id in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(Edge { id, to });
        g[to].push(Edge { id, to: fr });
    }

    let mut is_bridge = vec![false; m];
    let mut used = vec![false; n];

    let mut timer = 0;
    let mut tin = vec![0; n];
    let mut fup = vec![0; n];

    let mut dfs = RecursiveFunction2::new(|f, v: usize, p: usize| {
        if used[v] {
            return;
        }
        used[v] = true;
        tin[v] = timer;
        fup[v] = timer;
        timer += 1;
        for e in g[v].iter() {
            let to = e.to;
            if to == p {
                continue;
            }
            if used[to] {
                fup[v].update_min(tin[to]);
            } else {
                f.call(to, v);
                let tmp = fup[to];
                fup[v].update_min(tmp);
                if fup[to] > tin[v] {
                    is_bridge[e.id] = true;
                }
            }
        }
    });

    for root in 0..n {
        dfs.call(root, root);
    }

    let mut dsu = Dsu::new(n);
    for v in 0..n {
        for e in g[v].iter() {
            if !is_bridge[e.id] {
                dsu.unite(v, e.to);
            }
        }
    }

    let mut sz = vec![0; n];
    for v in 0..n {
        sz[dsu.get(v)] += 1;
    }

    const MAX_DIST: i32 = std::i32::MAX / 3;

    let get_dist = |start: usize| {
        let mut dist = vec![MAX_DIST; n];
        dist[start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            let cur_dist = dist[v];
            for e in g[v].iter() {
                if dist[e.to] > cur_dist + 1 {
                    dist[e.to] = cur_dist + 1;
                    queue.push_back(e.to);
                }
            }
        }
        return dist;
    };

    let from_queen = get_dist(queen_start);
    let from_alice = get_dist(alice_start);

    let mut inf = false;
    let mut res = 0;
    for v in 0..n {
        if from_alice[v] != MAX_DIST && from_queen[v] == MAX_DIST {
            inf = true;
        }
        if from_alice[v] < from_queen[v] {
            if sz[dsu.get(v)] != 1 {
                inf = true;
            } else {
                res.update_max(from_queen[v] * 2);
            }
        }
    }

    out!(format!("Case #{}: ", test_case));
    if inf {
        out_line!("SAFE");
    } else {
        out_line!(res);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
}
//END MAIN
