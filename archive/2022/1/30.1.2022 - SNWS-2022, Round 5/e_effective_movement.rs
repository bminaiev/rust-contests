//{"name":"E. Effective Movement","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"2 0 2 2\n","output":"2.0000000000\n"},{"input":"21 1 20 22\n","output":"21.4142135624\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEffectiveMovement"}}}

use algo_lib::graph::dijkstra::dijkstra;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::weighted_graph::WeightedGraph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::{dbg, out, out_line};
use std::cmp::{max, min};


// ML...

fn solve(input: &mut Input, _test_case: usize) {
    const M: usize = 307;
    let conv = |x: usize, y: usize| (x) * (M + 2) + (y);
    let x1 = input.usize();
    let y1 = input.usize();
    let x2 = input.usize();
    let y2 = input.usize();
    let p1 = conv(x1, y1);
    let p2 = conv(x2, y2);

    let mut graph = WeightedGraph::new((M + 4) * (M + 4));

    for x in (1..M - 3).step_by(2) {
        for y1 in 0..M - 3 {
            for y2 in 0..M - 3 {
                let dx = max(y1, y2) - min(y1, y2);
                let dx = dx as f64;
                let dy = 1 as f64;
                let d = (dx * dx + dy * dy).sqrt();
                let p1 = conv(x, y1);
                let p2 = conv(x + 1, y2);
                graph.add_bi_edge(p1, WeightedEdge::new(p2, OrdF64(d)));
                let p3 = conv(y1, x);
                let p4 = conv(y2, x + 1);
                graph.add_bi_edge(p3, WeightedEdge::new(p4, OrdF64(d)));
            }
        }
    }
    for x in 0..M - 3 {
        for y in 0..M - 3 {
            let p1 = conv(x, y);
            let p2 = conv(x, y + 1);
            let p3 = conv(x + 1, y);
            graph.add_bi_edge(p1, WeightedEdge::new(p2, OrdF64::ONE));
            graph.add_bi_edge(p1, WeightedEdge::new(p3, OrdF64::ONE));
        }
    }

    let dists = dijkstra(&graph, p1);
    let d = dists[p2];
    out_line!(d.dist);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
