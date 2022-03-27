//{"name":"F - Endless Walk","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_f","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1 2\n2 3\n3 4\n4 2\n4 5\n","output":"4\n"},{"input":"3 2\n1 2\n2 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEndlessWalk"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::strongly_connected_components::find_strongly_connected_component;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let graph = read_graph(input, n, m, Directional::Directed, Indexation::FromOne);
    let comps_struct = find_strongly_connected_component::<_, usize>(&graph);
    let comps = comps_struct.generate_components();
    let mut good_comp = gen_vec(comps.len(), |id| comps[id].len() > 1);

    let mut res = 0;
    for comp_id in (0..comps.len()).rev() {
        for &v in comps[comp_id].iter() {
            for e in graph.adj(v) {
                if good_comp[comps_struct.comp_id[e.to()]] {
                    good_comp[comp_id] = true;
                }
            }
        }
        if good_comp[comp_id] {
            res += comps[comp_id].len();
        }
    }

    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
