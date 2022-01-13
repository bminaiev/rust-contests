//{"name":"G. Дерево минимального ора","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n\n3 3\n1 2 1\n2 3 2\n1 3 2\n\n5 7\n4 2 7\n2 5 8\n3 4 2\n3 2 1\n2 4 2\n4 1 2\n1 2 2\n\n3 4\n1 2 1\n2 3 2\n1 3 3\n3 1 4\n","output":"2\n10\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GDerevoMinimalnogoOra"}}}

use algo_lib::graph::compressed_graph::CompressedGraph;
use algo_lib::graph::dsu::Dsu;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::weighted::read_weighted_graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let graph: CompressedGraph<WeightedEdge<u32>> =
        read_weighted_graph(input, n, m, Directional::Undirected, Indexation::FromOne);
    let mut res = (1u32 << 31) - 1;
    for bit in (0..31).rev() {
        let check = res ^ (1 << bit);
        let mut dsu = Dsu::new(n);
        for (fr, edge) in graph.all_edges() {
            if (edge.cost & check) == edge.cost {
                dsu.unite(fr, edge.to());
            }
        }
        if dsu.num_components() == 1 {
            res = check;
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
