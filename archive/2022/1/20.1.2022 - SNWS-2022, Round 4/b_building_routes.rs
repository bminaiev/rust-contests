//{"name":"B. Building Routes","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/B/","interactive":false,"timeLimit":5000,"tests":[{"input":"5 4\n4 1 s\n3 2 n\n1 2 w\n5 4 s\n3 2\n1 3\n2 1\n1 4\n","output":"0\n3\n1\n0\n"},{"input":"10 8\n1 2 a\n2 3 b\n3 4 c\n4 5 d\n5 6 e\n6 7 f\n7 8 g\n8 9 h\n9 10 i\n2 3\n3 2\n10 1\n3 5\n7 8\n10 9\n5 4\n4 8\n","output":"1\n0\n8\n3\n6\n0\n0\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBuildingRoutes"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::edge_with_info::EdgeWithInfo;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::rec_function::{Callable, Callable2, RecursiveFunction, RecursiveFunction2};
use algo_lib::{dbg, out, out_line};

type Edge = EdgeWithInfo<u8>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut graph = SimpleGraphT::new(n);
    for _ in 0..n - 1 {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let c = input.string_as_vec()[0];
        graph.add_edge(fr, Edge::new(to, c));
        graph.add_edge(to, Edge::new(fr, c));
    }
    let mut ans = Array2D::new(0, n, n);
    let mut cur_seen = vec![0i32; n];
    let mut by_lvl = vec![vec![]; n + 1];
    for root in 0..n {
        let mut seen = vec![false; n];
        by_lvl[0].push(root);
        RecursiveFunction::new(|f, lvl: usize| {
            let set = &by_lvl[lvl];
            for &v in set.iter() {
                ans[root][v] = cur_seen[root];
                seen[v] = true;
            }
            cur_seen[root] += set.len() as i32;
            for c in b'a'..=b'z' {
                for pos in 0..by_lvl[lvl].len() {
                    let v = by_lvl[lvl][pos];
                    for edge in graph.adj(v) {
                        if edge.info != c {
                            continue;
                        }
                        let to = edge.to();
                        if seen[to] {
                            continue;
                        }
                        by_lvl[lvl + 1].push(to);
                    }
                }
                if !by_lvl[lvl + 1].is_empty() {
                    f.call(lvl + 1);
                }
            }
            by_lvl[lvl].clear();
        })
        .call(0);
    }
    for _ in 0..q {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let res = if ans[fr][to] == 0 { 0 } else { ans[fr][to] - 1 };
        out_line!(res);
    }
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
}
//END MAIN
