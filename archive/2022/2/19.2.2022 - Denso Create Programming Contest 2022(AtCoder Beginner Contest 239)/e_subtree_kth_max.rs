//{"name":"E - Subtree K-th Max","group":"AtCoder - Denso Create Programming Contest 2022(AtCoder Beginner Contest 239)","url":"https://atcoder.jp/contests/abc239/tasks/abc239_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n1 2 3 4 5\n1 4\n2 1\n2 5\n3 2\n1 2\n2 1\n","output":"4\n5\n"},{"input":"6 2\n10 10 10 9 8 8\n1 4\n2 1\n2 5\n3 2\n6 4\n1 4\n2 2\n","output":"9\n10\n"},{"input":"4 4\n1 10 100 1000\n1 2\n2 3\n3 4\n1 4\n2 3\n3 2\n4 1\n","output":"1\n10\n100\n1000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESubtreeKThMax"}}}

use algo_lib::graph::bfs::bfs;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let q = input.usize();
    let xs = input.vec::<i32>(n);
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let bfs = bfs(0, &graph);
    let mut res = gen_vec(n, |v| vec![xs[v]]);
    for &v in bfs.queue.iter().rev() {
        for e in graph.adj(v) {
            let to = e.to();
            if to == bfs.prev[v] {
                continue;
            }
            let from = res[to].clone();
            res[v].extend(&from);
        }
        res[v].sort();
        res[v].reverse();
        res[v].truncate(20);
    }
    for _ in 0..q {
        let v = input.usize() - 1;
        let pos = input.usize() - 1;
        out_line!(res[v][pos]);
    }
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
    // tester::run_single_test("1");
}
//END MAIN
