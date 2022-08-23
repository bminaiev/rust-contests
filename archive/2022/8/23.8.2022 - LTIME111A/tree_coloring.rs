//{"name":"Tree Coloring","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems/TREECLR","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n1 2\n1 3\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TreeColoring"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::rec_function::{Callable2, Callable3, RecursiveFunction2, RecursiveFunction3};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let colors = input.usize();
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let mut dfs = RecursiveFunction3::new(|f, v: usize, p: usize, forb: usize| {
        if forb >= colors {
            return Mod::ZERO;
        }
        let mut res = Mod::new(colors - forb);
        let mut new_forb = 1;
        if p != v {
            new_forb += 1;
        }
        for e in graph.adj(v) {
            let to = e.to();
            if to == p {
                continue;
            }
            let w = f.call(to, v, new_forb);
            res *= w;
            new_forb += 1;
        }
        return res;
    });
    let res = dfs.call(0, 0, 0);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
