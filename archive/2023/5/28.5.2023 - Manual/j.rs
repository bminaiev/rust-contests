//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use algo_lib::collections::min_priority_queue::MinPriorityQueue;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Edge = WeightedEdge<i64>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Vertex {
    dist: i64,
    v: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize();
    let exits = input.vec::<usize>(k).sub_from_all(1);
    let mut monsters = input.vec::<usize>(n);
    for &e in exits.iter() {
        monsters[e] = 0;
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let w = input.i64();
        g[fr].push(Edge::new(to, w));
        g[to].push(Edge::new(fr, w));
    }
    let mut pq = MinPriorityQueue::new();
    for &e in exits.iter() {
        pq.push(Vertex { dist: 0, v: e });
    }
    let mut dist = vec![std::i64::MAX; n];
    while let Some(Vertex { dist: d, v }) = pq.pop() {
        if dist[v] != std::i64::MAX {
            continue;
        }
        if monsters[v] > 0 {
            monsters[v] -= 1;
        } else {
            dist[v] = d;
            for e in g[v].iter() {
                if dist[e.to()] == std::i64::MAX {
                    pq.push(Vertex {
                        dist: d + e.cost,
                        v: e.to(),
                    });
                }
            }
        }
    }
    if dist[0] == std::i64::MAX {
        out_line!(-1);
    } else {
        out_line!(dist[0]);
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
