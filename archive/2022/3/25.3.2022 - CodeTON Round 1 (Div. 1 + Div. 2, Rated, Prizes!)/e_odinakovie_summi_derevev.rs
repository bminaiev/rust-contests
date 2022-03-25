//{"name":"E. Одинаковые суммы деревьев","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n1 2\n1 3\n3 4\n3 5\n3\n1 2\n1 3\n","output":"-3 5 1 2 2\n1 1 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EOdinakovieSummiDerevev"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let mut root = 0;
    while graph.adj(root).len() != 1 {
        root += 1;
    }
    let mut res = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, v: usize, p: usize, exp_sum: i32| {
        let mut cur_sum = 0;
        for e in graph.adj(v) {
            let to = e.to();
            if to == p {
                continue;
            }
            cur_sum += -exp_sum;
            f.call(to, v, -exp_sum);
        }
        res[v] = exp_sum - cur_sum;
        assert_ne!(res[v], 0);
    });
    dfs.call(root, root, 1);
    res[root] = 1;
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

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
