//{"name":"D. Граф? А... А я думал, барон...","group":"Codeforces - Codeforces Round #791 (Div. 2)","url":"https://codeforces.com/contest/1679/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6 7 4\n1 10 2 3 4 5\n1 2\n1 3\n3 4\n4 5\n5 6\n6 2\n2 5\n","output":"4\n"},{"input":"6 7 100\n1 10 2 3 4 5\n1 2\n1 3\n3 4\n4 5\n5 6\n6 2\n2 5\n","output":"10\n"},{"input":"2 1 5\n1 1\n1 2\n","output":"-1\n"},{"input":"1 0 1\n1000000000\n","output":"1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGrafAAYaDumalBaron"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let moves = input.i64();
    let a = input.vec::<i64>(n);
    let graph = read_graph(input, n, m, Directional::Directed, Indexation::FromOne);
    const MAX: i64 = 2e9 as i64;
    let res = binary_search_first_true(0..MAX, |max_ok_value| -> bool {
        let mut max_d = vec![-1; n];
        let mut dfs = RecursiveFunction::new(|f, v: usize| -> i64 {
            if a[v] > max_ok_value {
                return 0;
            }
            if max_d[v] != -1 {
                return max_d[v];
            }
            max_d[v] = std::i64::MAX / 2;
            let mut d = 1;
            for e in graph.adj(v) {
                let to = e.to();
                let d_to = f.call(to) + 1;
                if d_to > d {
                    d = d_to;
                }
            }
            max_d[v] = d;
            d
        });
        (0..n).any(|v| dfs.call(v) >= moves)
    });
    if res == MAX {
        out_line!(-1);
    } else {
        out_line!(res);
    }
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
}
//END MAIN
