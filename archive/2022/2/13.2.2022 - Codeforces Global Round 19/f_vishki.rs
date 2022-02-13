//{"name":"F. Вышки","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 1\n1 2\n2 3\n","output":"4\n"},{"input":"5\n1 3 3 1 3\n1 3\n5 4\n4 3\n2 3\n","output":"7\n"},{"input":"2\n6 1\n1 2\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FVishki"}}}

use std::cmp::max;

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::FindMinMaxPos;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let h = input.read_vec::<i64>(n);
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let root = h.index_of_max();
    let mut res = 0;
    RecursiveFunction2::new(|f, v, p| -> i64 {
        let mut from_child = vec![];
        for edge in graph.adj(v) {
            let to = edge.to();
            if to == p {
                continue;
            }
            from_child.push(f.call(to, v));
        }
        let my_h = h[v];
        let biggest_child = *from_child.iter().max().unwrap_or(&0);
        let add = max(0, my_h - biggest_child);
        res += add;
        if v == p {
            from_child.sort();
            let second_biggest = if from_child.len() < 2 {
                0
            } else {
                from_child[from_child.len() - 2]
            };
            let add_more = my_h - second_biggest;
            res += add_more;
            my_h
        } else {
            biggest_child + add
        }
    })
    .call(root, root);
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
}
//END MAIN
