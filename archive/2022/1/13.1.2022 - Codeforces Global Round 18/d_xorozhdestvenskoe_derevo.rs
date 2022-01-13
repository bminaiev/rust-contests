//{"name":"D. (XO)R-ождественское дерево","group":"Codeforces - Codeforces Global Round 18","url":"https://codeforces.com/contest/1615/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6 5\n1 2 -1\n1 3 1\n4 2 7\n6 3 0\n2 5 -1\n2 3 1\n2 5 0\n5 6 1\n6 1 1\n4 5 1\n5 3\n1 2 -1\n1 3 -1\n1 4 1\n4 5 -1\n2 4 0\n3 4 1\n2 3 1\n3 3\n1 2 -1\n1 3 -1\n1 2 0\n1 3 1\n2 3 0\n2 1\n1 2 1\n1 2 0\n","output":"YES\n1 2 0\n1 3 1\n2 4 7\n3 6 0\n2 5 0\nYES\n1 2 1\n1 3 0\n1 4 1\n4 5 1\nNO\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DXOROzhdestvenskoeDerevo"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};

use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut graph = SimpleGraphT::<WeightedEdge<i32>>::new(n);
    let edges = gen_vec(n - 1, |_| {
        (input.usize() - 1, input.usize() - 1, input.i32())
    });
    for &(fr, to, value) in edges.iter() {
        if value == -1 {
            continue;
        }
        graph.add_edge(fr, WeightedEdge::new(to, (value.count_ones() % 2) as i32));
        graph.add_edge(to, WeightedEdge::new(fr, (value.count_ones() % 2) as i32));
    }
    for _ in 0..m {
        let x = input.usize() - 1;
        let y = input.usize() - 1;
        let cnt = input.i32();
        graph.add_edge(x, WeightedEdge::new(y, cnt));
        graph.add_edge(y, WeightedEdge::new(x, cnt));
    }
    let mut seen = vec![false; n];
    let mut xor = vec![0; n];
    for root in 0..n {
        if seen[root] {
            continue;
        }
        RecursiveFunction::new(|f, v| {
            seen[v] = true;
            for edge in graph.adj(v) {
                if !seen[edge.to()] {
                    xor[edge.to()] = xor[v] ^ edge.cost;
                    f.call(edge.to());
                }
            }
        })
        .call(root);
    }
    for (fr, edge) in graph.all_edges() {
        if xor[fr] ^ xor[edge.to()] != edge.cost {
            out_line!("NO");
            return;
        }
    }
    out_line!("YES");
    for &(fr, to, value) in edges.iter() {
        let use_val = if value == -1 {
            xor[fr] ^ xor[to]
        } else {
            value
        };
        out_line!(fr + 1, to + 1, use_val);
    }
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
