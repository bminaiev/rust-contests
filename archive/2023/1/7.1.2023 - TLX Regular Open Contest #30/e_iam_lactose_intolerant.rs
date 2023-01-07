//{"name":"E. I am Lactose Intolerant","group":"TLX - TLX Regular Open Contest #30","url":"https://tlx.toki.id/contests/troc-30/problems/E","interactive":false,"timeLimit":3000,"tests":[{"input":"5 8\n1 5\n1 4\n2 1\n5 5\n4 5\n5 3\n2 3\n1 4\n4\n1 3\n4 5\n3 1\n2 2\n","output":"1000110\n"},{"input":"21 20\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n8 9\n9 10\n10 11\n11 12\n12 13\n13 14\n14 15\n15 16\n16 17\n17 18\n18 19\n19 20\n20 21\n1\n1 21\n","output":"111111111111111111110\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EIAmLactoseIntolerant"}}}

use std::collections::VecDeque;

use algo_lib::graph::dsu::Dsu;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::graph::trees::binary_lifting::BinaryLifting;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();

    let mut edges = vec![];
    for _ in 0..m {
        edges.push((input.usize() - 1, input.usize() - 1));
    }

    let mut dsu = Dsu::new(n);
    let mut graph = SimpleGraphT::new(n);
    for i in 0..m {
        let (fr, to) = edges[i];
        if dsu.unite(fr, to) {
            graph.add_bi_weighted_edge(fr, to, i);
        }
    }

    let lca = BinaryLifting::new(&graph, 0);

    let k = input.usize();
    let mut delta = vec![0i32; n];
    for _ in 0..k {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        delta[fr] += 1;
        delta[to] += 1;
        delta[lca.lca(fr, to)] -= 2;
    }

    let mut queue = vec![];
    queue.push(0);
    let mut seen = vec![false; n];
    let mut iter = 0;
    let mut parent = vec![None; n];
    while iter < queue.len() {
        let v = queue[iter];
        iter += 1;
        seen[v] = true;
        for e in graph.adj(v) {
            if seen[e.to()] {
                continue;
            }
            parent[e.to()] = Some((v, e.cost));
            queue.push(e.to());
        }
    }
    let mut used = vec![false; m];
    for &v in queue.iter().rev() {
        if let Some((p, id)) = parent[v] {
            if delta[v] != 0 {
                used[id] = true;
                delta[p] += delta[v];
            }
        }
    }
    let mut write = false;
    for cost in (0..used.len()).rev() {
        if used[cost] || write {
            write = true;
            out!(used[cost] as u32);
        }
    }
    out_line!(0);
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
