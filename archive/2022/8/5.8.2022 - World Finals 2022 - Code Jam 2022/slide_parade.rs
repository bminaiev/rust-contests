//{"name":"Slide Parade","group":"Google Coding Competitions - World Finals 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/000000000087762e/0000000000b9cb13","interactive":false,"timeLimit":10000,"tests":[{"input":"5\n2 2\n2 1\n1 2\n3 4\n2 3\n1 2\n3 2\n1 3\n3 6\n1 2\n1 3\n2 1\n2 3\n3 1\n3 2\n3 4\n1 2\n2 1\n1 3\n3 1\n4 6\n1 2\n1 4\n2 3\n3 2\n3 4\n4 1\n","output":"Case #1: 7\n1 2 1 2 1 2 1\nCase #2: IMPOSSIBLE\nCase #3: 7\n1 2 3 1 3 2 1\nCase #4: IMPOSSIBLE\nCase #5: 9\n1 4 1 2 3 2 3 4 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SlideParade"}}}

use algo_lib::flows::dinic::FlowDinic;
use algo_lib::graph::bfs::bfs;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let g = read_graph(input, n, m, Directional::Directed, Indexation::FromOne);

    let from_zero = bfs(0, &g);
    let mut impossible = false;
    for v in 1..n {
        if from_zero.dist[v] == std::u32::MAX {
            impossible = true;
        }
    }
    for v in 1..n {
        let from_here = bfs(v, &g);
        if from_here.dist[0] == std::u32::MAX {
            impossible = true;
        }
    }
    let mut times = 1;
    for v in 0..n {
        times.update_max(g.adj(v).len());
    }
    let mut atleast_inside = vec![0; n];
    for v in 0..n {
        for e in g.adj(v) {
            atleast_inside[e.to()] += 1;
        }
    }
    for v in 0..n {
        times.update_max(atleast_inside[v]);
    }
    times.update_max(1e6 as usize / m);
    let mut res = None;
    if !impossible {
        let mut dinic = FlowDinic::new(1 + n + n + 1);
        let mut exp_flow = 0;
        let mut ids = vec![vec![]; n];
        let mut ids_to_update = vec![];
        for v in 0..n {
            let fl = (times - g.adj(v).len()) as i64;
            exp_flow += fl;
            ids_to_update.push(dinic.add_edge(0, 1 + v, fl));
            ids_to_update.push(dinic.add_edge(
                1 + n + v,
                1 + n + n,
                (times - atleast_inside[v]) as i64,
            ));
            for e in g.adj(v) {
                ids[v].push(dinic.add_edge(1 + v, 1 + n + e.to(), std::i64::MAX / 10));
            }
        }
        let mut total_flow = 0;

        {
            total_flow += dinic.find_flow();
            if total_flow == exp_flow {
                let mut ng = vec![vec![]; n];
                for v in 0..n {
                    let mut it = 0;
                    for e in g.adj(v) {
                        let cur_id = ids[v][it];
                        let flow = dinic.get_edge_flow(cur_id) + 1;
                        for _ in 0..flow {
                            ng[v].push(e.to());
                        }
                        it += 1;
                    }
                }
                let mut iter = vec![0; n];
                let mut stack = vec![0];
                let mut path = vec![];
                while let Some(v) = stack.pop() {
                    if iter[v] == ng[v].len() {
                        path.push(v + 1);
                    } else {
                        stack.push(v);
                        let next = ng[v][iter[v]];
                        iter[v] += 1;
                        stack.push(next);
                    }
                }
                assert_eq!(path.len() - 1, times * n);
                if path.len() <= 1_000_001 {
                    path.reverse();
                    res = Some(path);
                }
            }
        }
    }
    out!(format!("Case #{}: ", test_case));
    if let Some(path) = res {
        out_line!(path.len());
        out_line!(path);
    } else {
        out_line!("IMPOSSIBLE");
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
