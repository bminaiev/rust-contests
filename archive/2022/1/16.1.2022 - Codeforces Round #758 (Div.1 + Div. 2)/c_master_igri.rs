//{"name":"C. Мастер игры","group":"Codeforces - Codeforces Round #758 (Div.1 + Div. 2)","url":"https://codeforces.com/contest/1608/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 2 3 4\n1 2 3 4\n4\n11 12 20 21\n44 22 11 30\n1\n1000000000\n1000000000\n","output":"0001\n1111\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMasterIgri"}}}

use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::graph::strongly_connected_components::find_strongly_connected_component;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i32>(n);
    let b = input.read_vec::<i32>(n);
    let mut ids = gen_vec(n, |x| x);
    let mut graph = SimpleGraphT::<SimpleEdge>::new(n);
    for by in [a, b].iter() {
        ids.sort_by_key(|&id| by[id]);
        for w in ids.windows(2) {
            graph.add_edge(w[1], SimpleEdge::new(w[0]));
        }
    }
    let comp_ids = find_strongly_connected_component(&graph);
    for id in 0..n {
        if comp_ids[id] == 0 {
            out!(1)
        } else {
            out!(0);
        }
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
