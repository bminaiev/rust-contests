//{"name":"E. Buy and Delete","group":"Yandex - Stage 6: Grand Prix of EDG","url":"https://official.contest.yandex.ru/opencupXXII/contest/31241/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2 4\n1 2 5\n2 3 6\n","output":"0\n"},{"input":"3 3 3\n1 2 1\n2 3 1\n1 3 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBuyAndDelete"}}}

use algo_lib::graph::dijkstra::dijkstra;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::GraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

type Edge = WeightedEdge<i64>;
type Graph = GraphT<Edge>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let c = input.i64();

    let mut graph = Graph::new(n);
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let cost = input.i64();
        graph.add_edge(fr, WeightedEdge::new(to, cost));
    }

    let any = graph.all_edges().any(|(_from, edge)| edge.cost <= c);

    let exist_cycle = (0..n).any(|root| -> bool {
        let vertices = dijkstra(&graph, root);
        graph.all_edges().any(|(from, edge)| {
            edge.to() == root
                && vertices[from].dist != i64::MAX
                && vertices[from].dist + edge.cost <= c
        })
    });

    let res = if exist_cycle {
        2
    } else if any {
        1
    } else {
        0
    };
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
