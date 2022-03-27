//{"name":"G - Foreign Friends","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_g","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 2 2\n1 1 2 2\n2 3\n1 2 15\n2 3 30\n3 4 40\n1 4 10\n","output":"45 30 30 25\n"},{"input":"3 1 3 1\n1 2 3\n1\n1 2 1000000000\n","output":"-1 1000000000 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GForeignFriends"}}}

use std::collections::BTreeSet;

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::two_min::TwoMin;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Vertex {
    cost: i64,
    v: usize,
    id: usize,
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let _num_nations = input.usize();
    let num_popular = input.usize();
    let nation = input.vec::<usize>(n);
    let popular = input.vec::<usize>(num_popular).sub_from_all(1);
    let mut graph = SimpleGraphT::new(n);
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let cost = input.i64();
        graph.add_bi_weighted_edge(fr, to, cost);
    }
    let mut mins = vec![TwoMin::new(0, 0); n];
    let mut pq = BTreeSet::new();
    for &v in popular.iter() {
        if mins[v].add(nation[v], 0) {
            pq.insert(Vertex {
                cost: 0,
                v,
                id: nation[v],
            });
        }
    }
    while let Some(vertex) = pq.iter().next() {
        let vertex = vertex.clone();
        pq.remove(&vertex);
        if mins[vertex.v].get_value_by_id(vertex.id) != Some(vertex.cost) {
            continue;
        }
        for e in graph.adj(vertex.v) {
            let to = e.to();
            if mins[to].add(vertex.id, vertex.cost + e.cost) {
                pq.insert(Vertex {
                    cost: vertex.cost + e.cost,
                    v: e.to(),
                    id: vertex.id,
                });
            }
        }
    }

    for v in 0..n {
        if let Some(res) = mins[v].get_value_by_not_id(nation[v]) {
            out!(res, "");
        } else {
            out!("-1 ");
        }
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("2");
}
//END MAIN
