//{"name":"E - Skiing","group":"AtCoder - AtCoder Beginner Contest 237","url":"https://atcoder.jp/contests/abc237/tasks/abc237_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4\n10 8 12 5\n1 2\n1 3\n2 3\n3 4\n","output":"3\n"},{"input":"2 1\n0 10\n1 2\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESkiing"}}}

use algo_lib::graph::dijkstra_with_potentials::dijkstra_with_potentials;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::weighted_graph::WeightedGraph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let h = input.read_vec::<i64>(n);
    let potential = gen_vec(n, |id| -h[id]);
    let mut graph = WeightedGraph::new(n);
    let mut add_edge = |x: usize, y: usize| {
        let w = if h[x] >= h[y] {
            -(h[x] - h[y])
        } else {
            2 * (h[y] - h[x])
        };
        graph.add_edge(x, WeightedEdge::new(y, w));
    };
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        add_edge(fr, to);
        add_edge(to, fr);
    }
    let dists = dijkstra_with_potentials(&graph, 0, &potential);
    let mut res = 0;
    for v in 0..n {
        res.update_min(dists[v].dist);
    }
    out_line!(-res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
