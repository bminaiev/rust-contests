//{"name":"D. Эла и волшебник","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n8 9\n1 2 3\n6 4 5\n3 5 6\n6 1 3\n7 4 4\n3 8 4\n2 3 3\n7 8 5\n4 5 2\n4 5\n1 2 1\n2 4 1\n3 4 1\n3 1 1\n1 3 2\n8 8\n4 6 92\n7 1 65\n6 5 43\n6 7 96\n4 3 74\n4 8 54\n7 4 99\n2 5 22\n","output":"9\n2\n154\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DElaIVolshebnik"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::graph::bfs::bfs;
use algo_lib::graph::compressed_graph::CompressedGraph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::weighted::read_weighted_graph;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Edge = WeightedEdge<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let graph: CompressedGraph<Edge> =
        read_weighted_graph(input, n, m, Directional::Undirected, Indexation::FromOne);
    let mut res = std::i64::MAX;
    let bfs0 = bfs(0, &graph);
    let bfs_n = bfs(n - 1, &graph);
    const NOT_REACHABLE: u32 = std::u32::MAX;

    let mut g = Array2D::new(NOT_REACHABLE / 2, n, n);
    for i in 0..n {
        g[i][i] = 0;
    }
    for (fr, edge) in graph.all_edges() {
        g[fr][edge.to()] = 1;
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let c = g[j][i] + g[i][k];
                g[j][k].update_min(c);
            }
        }
    }
    let mut to_path = vec![NOT_REACHABLE; n];
    for root in 0..n {
        for v in 0..n {
            let cur = g[root][0] + g[root][n - 1] + g[root][v] + 2;
            to_path[v].update_min(cur);
        }
    }

    for (fr, edge) in graph.all_edges() {
        let d1 = bfs0.dist[fr];
        let d3 = bfs_n.dist[edge.to()];
        let cnt = (d1 + d3 + 1) as i64;
        res.update_min(cnt * edge.cost);
        {
            let cnt = to_path[fr] as i64;
            res.update_min(cnt * edge.cost);
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
