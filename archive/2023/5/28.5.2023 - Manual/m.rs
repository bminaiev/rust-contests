//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

use std::collections::{HashSet, VecDeque};

use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::graph::strongly_connected_components::{
    find_strongly_connected_component, StronglyConnectedComponents,
};
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Edge {
    fr: usize,
    to: usize,
    id: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();

    let mut values = vec![0; n];
    let mut check_values = vec![0; n];
    let mut res_rev = vec![];
    let mut last_res = vec![];

    let mut edges = vec![vec![]; n];

    for id in 0..m {
        let l = input.usize() - 1;
        let x = input.usize();
        let r = input.usize() - 1;
        let y = input.usize();

        if x == 2 && y == 2 {
            values[l] = 2;
            values[r] = 2;
            check_values[l] = 2;
            check_values[r] = 2;
            res_rev.push(id);
        } else if x == 1 && y == 1 {
            last_res.push(Edge { fr: l, to: r, id });
        } else if x == 1 {
            edges[l].push(Edge { fr: l, to: r, id })
        } else {
            edges[r].push(Edge { fr: r, to: l, id })
        }
    }

    let mut graph = SimpleGraphT::new(n);
    for fr in 0..n {
        for e in edges[fr].iter() {
            graph.add_edge(e.fr, e.to);
        }
    }

    let comps: StronglyConnectedComponents<usize> = find_strongly_connected_component(&graph);
    let mut by_id = vec![vec![]; comps.num_comps];
    for v in 0..n {
        by_id[comps.comp_id[v]].push(v);
    }
    let mut seen_comp = vec![false; comps.num_comps];
    for v in 0..n {
        if values[v] == 2 {
            seen_comp[comps.comp_id[v]] = true;
        }
    }
    let mut used_edges = HashSet::new();
    for comp_id in 0..seen_comp.len() {
        let mut found_any = false;
        if !seen_comp[comp_id] {
            for &v in by_id[comp_id].iter() {
                if edges[v].len() != 0 {
                    found_any = true;
                    values[v] = 1;
                    check_values[v] = 1;
                    res_rev.push(edges[v][0].id);
                    used_edges.insert(edges[v][0].id);
                    break;
                }
            }
        } else {
            found_any = true;
        }
        for &v in by_id[comp_id].iter() {
            if found_any && values[v] == 0 {
                values[v] = 2;
            }
            for e in edges[v].iter() {
                let to = e.to;
                let to_comp = comps.comp_id[to];
                seen_comp[to_comp] = true;
            }
        }
    }
    for e in last_res.iter() {
        if values[e.fr] == 0 {
            values[e.fr] = 1;
        }
        if values[e.to] == 0 {
            values[e.to] = 1;
        }
    }
    let mut queue = VecDeque::new();
    for v in 0..n {
        if check_values[v] == 2 || check_values[v] == 1 {
            queue.push_back(v);
        }
    }
    while let Some(v) = queue.pop_front() {
        for e in edges[v].iter() {
            if !used_edges.contains(&e.id) {
                res_rev.push(e.id);
            }
            let to = e.to;
            if check_values[to] == 0 {
                check_values[to] = 2;
                queue.push_back(to);
            }
        }
    }
    for e in last_res.iter() {
        res_rev.push(e.id);
        if check_values[e.fr] == 0 {
            check_values[e.fr] = 1;
        }
        if check_values[e.to] == 0 {
            check_values[e.to] = 1;
        }
    }
    for v in 0..n {
        assert_eq!(check_values[v], values[v]);
    }
    res_rev.reverse();
    for i in 0..res_rev.len() {
        res_rev[i] += 1;
    }
    let sum = check_values.iter().sum::<usize>();
    out_line!(sum);
    out_line!(res_rev);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
