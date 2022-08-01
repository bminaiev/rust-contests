//{"name":"A. DFS Order","group":"Yandex - Stage 19: Grand Prix of China","url":"https://official.contest.yandex.com/opencupXXII/contest/39025/problems/","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n4\n1 2\n2 3\n3 4\n5\n1 2\n2 3\n2 4\n1 5\n","output":"1 1\n2 2\n3 3\n4 4\n1 1\n2 3\n3 5\n3 5\n2 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADFSOrder"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable2, Callable4, RecursiveFunction2, RecursiveFunction4};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let g = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let mut res = vec![(0, 0); n];
    let mut sizes = vec![1; n];
    RecursiveFunction2::new(|f, v, p| {
        for e in g.adj(v) {
            let to = e.to();
            if to == p {
                continue;
            }
            f.call(to, v);
            sizes[v] += sizes[to];
        }
    })
    .call(0, 0);
    RecursiveFunction4::new(|f, v, p, min_v, max_v| {
        let whole_size = sizes[v];
        res[v] = (min_v, max_v);
        for e in g.adj(v) {
            let to = e.to();
            if to == p {
                continue;
            }
            f.call(to, v, min_v + 1, max_v + whole_size - sizes[to]);
        }
    })
    .call(0, 0, 0, 0);
    for (a, b) in res.into_iter() {
        out_line!(a + 1, b + 1);
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
    // tester::run_stress(stress);
}
//END MAIN
