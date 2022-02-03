//{"name":"E. Запросы об остовном дереве","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5 8\n4 1 4\n3 1 0\n3 5 3\n2 5 4\n3 4 8\n4 3 4\n4 2 8\n5 3 9\n3 11 1 1 10\n0 1 2\n","output":"4\n"},{"input":"6 7\n2 4 0\n5 4 7\n2 4 0\n2 1 7\n2 6 1\n3 4 4\n1 4 8\n4 10 3 3 7\n3 0 2 1\n","output":"5\n"},{"input":"3 3\n1 2 50\n2 3 100\n1 3 150\n1 10000000 0 0 100000000\n75\n","output":"164\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZaprosiObOstovnomDereve"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::weighted::read_weighted_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::minimal_spanning_tree::minimal_spanning_tree;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::num_traits_tuple::number_pair;
use algo_lib::misc::vec_binary_search::VecBinarySearch;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let graph =
        read_weighted_graph::<i64>(input, n, m, Directional::Undirected, Indexation::FromOne);
    let queries = {
        let p = input.usize();
        let k = input.usize();
        let a = input.i64();
        let b = input.i64();
        let c = input.i64();
        let mut qs = input.read_vec::<i64>(p);
        qs.reserve(k - p);
        for _ in 0..k - p {
            let next = (*qs.last_exn() * a + b) % c;
            qs.push(next);
        }
        qs
    };
    let mut split = vec![0];
    for (_, e1) in graph.all_edges() {
        split.push(e1.cost + 1);
        for (_, e2) in graph.all_edges() {
            split.push((e1.cost + e2.cost) / 2 + 1);
        }
    }
    split.sort();
    split.dedup();
    let functions: Vec<_> = split
        .into_iter()
        .map(|x| {
            let mut new_graph = SimpleGraphT::new(n);
            for (fr, edge) in graph.all_edges() {
                let w = edge.cost;
                new_graph.add_weighted_edge(fr, edge.to(), number_pair((w - x).abs(), w));
            }
            let tree = minimal_spanning_tree(&new_graph);
            assert_eq!(tree.num_edges() + 1, n);
            let mut k = 0;
            let mut b = 0;
            // k * x + b
            for (_, edge) in tree.all_edges() {
                let real_w = edge.cost.second;
                if real_w >= x {
                    k -= 1;
                    b += real_w;
                } else {
                    k += 1;
                    b -= real_w;
                }
            }
            (x, k, b)
        })
        .collect();
    let mut res = 0;

    for query in queries.into_iter() {
        let (_, k, b) = functions.lower(&(query, i64::MAX, i64::MAX)).unwrap();
        let cur_ans = k * query + b;
        assert!(cur_ans >= 0);
        res ^= cur_ans;
    }
    out_line!(res);
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
