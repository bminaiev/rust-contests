//{"name":"F. Fast Climb","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"5 6\n0 3 3\n0 1 9\n3 2 12\n2 1 6\n2 4 2\n4 3 10\n","output":"7\n"},{"input":"7 8\n1 0 8\n3 6 0\n0 6 12\n2 1 6\n2 4 2\n4 3 10\n4 5 3\n4 4 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFastClimb"}}}

use algo_lib::graph::dijkstra::dijkstra;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::graph::weighted_graph::WeightedGraph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn conv(dist: usize) -> usize {
    if dist == 0 {
        return 0;
    }
    // dist
    let full = dist / 12;
    let more = dist % 12;
    if more == 0 {
        (full - 1) * 24 + 12
    } else {
        (full) * 24 + more
    }
}

fn simple(n: usize, edges: &[(usize, usize, usize)]) -> usize {
    if n == 1 {
        return 0;
    }
    let mut graph = WeightedGraph::new(n);
    for &(fr, to, cost) in edges.iter() {
        graph.add_edge(fr, WeightedEdge::new(to, cost));
        graph.add_edge(to, WeightedEdge::new(fr, cost));
    }

    let dist = dijkstra(&graph, 0)[n - 1].dist;
    conv(dist)
}

fn only_stay(n: usize, edges: &[(usize, usize, usize)]) -> usize {
    if n == 1 {
        return 0;
    }
    const M: usize = 12;
    let mut graph = WeightedGraph::new(n * M);
    for &(fr, to, cost) in edges.iter() {
        for offset in 0..M {
            if offset + cost <= M {
                graph.add_edge(
                    fr * M + offset,
                    WeightedEdge::new(to * M + (offset + cost) % M, cost),
                );
                graph.add_edge(
                    to * M + offset,
                    WeightedEdge::new(fr * M + (offset + cost) % M, cost),
                );
            } else {
                let add = M - offset;
                graph.add_edge(
                    fr * M + offset,
                    WeightedEdge::new(to * M + cost % M, add + cost),
                );
                graph.add_edge(
                    to * M + offset,
                    WeightedEdge::new(fr * M + cost % M, add + cost),
                );
            }
        }
    }

    let dists = dijkstra(&graph, 0);
    let mut res = usize::MAX;
    for offset in 0..M {
        let d = dists[(n - 1) * M + offset].dist;
        if d == usize::MAX {
            continue;
        }
        res.update_min(conv(d));
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let edges: Vec<(usize, usize, usize)> =
        gen_vec(m, |_| (input.usize(), input.usize(), input.usize()));
    let fast = simple(n, &edges);
    let slow = only_stay(n, &edges);
    // dbg!(fast, slow);
    out_line!(slow - fast);
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
