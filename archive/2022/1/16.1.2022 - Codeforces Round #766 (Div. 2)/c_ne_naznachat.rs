//{"name":"C. Не назначать","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n2\n1 2\n4\n1 3\n4 3\n2 1\n7\n1 2\n1 3\n3 4\n3 5\n6 2\n7 2\n","output":"17\n2 5 11\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNeNaznachat"}}}

use algo_lib::graph::bfs::bfs;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};
use std::cmp::min;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let edges = gen_vec(n - 1, |_| (input.usize() - 1, input.usize() - 1));
    let mut total_counts = vec![0; n];
    for &(fr, to) in edges.iter() {
        total_counts[fr] += 1;
        total_counts[to] += 1;
    }
    if total_counts.iter().any(|&sz| sz >= 3) {
        out_line!(-1);
        return;
    }
    let mut graph = SimpleGraphT::new(n);
    for &(fr, to) in edges.iter() {
        graph.add_edge(fr, SimpleEdge::new(to));
        graph.add_edge(to, SimpleEdge::new(fr));
    }
    let root = (0..n).min_by_key(|&v| total_counts[v]).unwrap();
    let bfs = bfs(root, &graph).dist;
    for &(fr, to) in edges.iter() {
        let min_d = min(bfs[fr], bfs[to]);
        let res = (min_d % 2) + 2;
        out!(res, "");
    }
    out_line!();
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
