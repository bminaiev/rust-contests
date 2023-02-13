//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::strongly_connected_components::{
    find_strongly_connected_component, StronglyConnectedComponents,
};
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let g = read_graph(input, n, m, Directional::Directed, Indexation::FromOne);
    let comps: StronglyConnectedComponents<usize> = find_strongly_connected_component(&g);
    let mut need_include = vec![true; comps.num_comps];
    for (fr, edge) in g.all_edges() {
        let c_fr = comps.comp_id[fr];
        let c_to = comps.comp_id[edge.to()];
        if c_fr != c_to {
            need_include[c_to] = false;
        }
    }
    let mut min_id = vec![usize::MAX; comps.num_comps];
    for v in (0..n).rev() {
        min_id[comps.comp_id[v]] = v;
    }
    let mut res = vec![];
    for i in 0..need_include.len() {
        if need_include[i] {
            res.push(min_id[i] + 1);
        }
    }
    res.sort();
    for &x in res.iter() {
        out_line!(x);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_single_test("3");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
