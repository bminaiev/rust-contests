//{"name":"G. Dynamic Reachability","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/G/","interactive":false,"timeLimit":12000,"tests":[{"input":"5 6 7\n1 2\n1 3\n2 4\n3 4\n3 5\n4 5\n2 1 5\n2 2 3\n1 3\n1 4\n2 1 4\n1 3\n2 1 5\n","output":"YES\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GDynamicReachability"}}}

use std::cmp::min;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::bit_set_fixed_size::BitSetFixedSize;
use algo_lib::graph::bfs_bitsets_fixed_size::bfs_bitsets_fixed_size;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::graph::strongly_connected_components::{
    find_strongly_connected_component, StronglyConnectedComponents,
};
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Edge {
    fr: usize,
    to: usize,
}

fn solve_test(n: usize, edges: &[Edge], queries: &[Op], buben: usize) -> Vec<bool> {
    let mut enabled = vec![true; edges.len()];
    let mut edge_will_be_used = vec![false; edges.len()];
    let mut is_interesting = vec![false; n];
    let mut interesting_id = vec![None; n];
    let mut res = vec![];
    let mut new_graph = SimpleGraphT::new(n);
    for start_q in (0..queries.len()).step_by(buben) {
        // dbg!(start_q);
        let ops = &queries[start_q..min(start_q + buben, queries.len())];
        for x in edge_will_be_used.iter_mut() {
            *x = false;
        }
        for op in ops.iter() {
            if let Op::ChangeColor(edge_id) = op {
                edge_will_be_used[*edge_id] = true;
            }
        }
        new_graph.clear();
        for i in 0..edges.len() {
            if !edge_will_be_used[i] && enabled[i] {
                new_graph.add_edge(edges[i].fr, edges[i].to);
            }
        }
        let con_comps: StronglyConnectedComponents<u32> =
            find_strongly_connected_component(&new_graph);
        let n_comps = con_comps.num_comps;
        let mut comps = vec![vec![]; n_comps];
        for v in 0..n {
            comps[con_comps.comp_id[v] as usize].push(v);
        }
        for x in is_interesting.iter_mut() {
            *x = false;
        }
        for op in ops.iter() {
            match op {
                Op::ChangeColor(edge_id) => {
                    is_interesting[edges[*edge_id].fr] = true;
                    is_interesting[edges[*edge_id].to] = true;
                }
                Op::CanReach(fr, to) => {
                    is_interesting[*fr] = true;
                    is_interesting[*to] = true;
                }
            }
        }
        let mut interestings = vec![];
        for v in 0..n {
            if is_interesting[v] {
                interesting_id[v] = Some(interestings.len());
                interestings.push(v);
            } else {
                interesting_id[v] = None;
            }
        }
        let sz = interestings.len();
        let mut by_comp = vec![BitSetFixedSize::new(sz); con_comps.num_comps];
        for comp_id in (0..con_comps.num_comps).rev() {
            for &v in comps[comp_id].iter() {
                if let Some(i_id) = interesting_id[v] {
                    by_comp[comp_id].set(i_id, true);
                }
                for e in new_graph.adj(v) {
                    let ncomp = con_comps.comp_id[e.to()] as usize;
                    assert!(ncomp >= comp_id);
                    if ncomp > comp_id {
                        let z = by_comp[ncomp].clone();
                        by_comp[comp_id] |= &z;
                    }
                }
            }
        }
        let mut full_bs_graph = vec![BitSetFixedSize::new(sz); sz];
        for i in 0..sz {
            let v = interestings[i];
            full_bs_graph[i] = by_comp[con_comps.comp_id[v] as usize].clone();
        }
        let mut bs_graph = full_bs_graph.clone();

        for op in ops.iter() {
            if let Op::ChangeColor(edge_id) = op {
                let edge_id = *edge_id;
                let v1 = interesting_id[edges[edge_id].fr].unwrap();
                let v2 = interesting_id[edges[edge_id].to].unwrap();
                if enabled[edge_id] {
                    bs_graph[v1].set(v2, true);
                }
            }
        }

        let mut cnt_enabled = Array2D::new(0i32, sz, sz);
        for edge_id in 0..edges.len() {
            if edge_will_be_used[edge_id] && enabled[edge_id] {
                let v1 = interesting_id[edges[edge_id].fr].unwrap();
                let v2 = interesting_id[edges[edge_id].to].unwrap();
                cnt_enabled[v1][v2] += 1;
            }
        }

        for op in ops.iter() {
            match op {
                Op::ChangeColor(edge_id) => {
                    let edge_id = *edge_id;
                    enabled[edge_id] = !enabled[edge_id];
                    let v1 = interesting_id[edges[edge_id].fr].unwrap();
                    let v2 = interesting_id[edges[edge_id].to].unwrap();
                    if enabled[edge_id] {
                        cnt_enabled[v1][v2] += 1;
                        bs_graph[v1].set(v2, true);
                    } else {
                        cnt_enabled[v1][v2] -= 1;
                        if !full_bs_graph[v1].get(v2) && cnt_enabled[v1][v2] == 0 {
                            bs_graph[v1].set(v2, false);
                        }
                    }
                }
                Op::CanReach(fr, to) => {
                    let v1 = interesting_id[*fr].unwrap();
                    let v2 = interesting_id[*to].unwrap();
                    let bfs = bfs_bitsets_fixed_size(v1, &bs_graph);
                    res.push(bfs.get(v2));
                }
            }
        }
    }
    res
}

// fn check_target_feature() {
//     if is_x86_feature_detected!("sse3") {
//         assert!(false);
//     }
//     #[cfg(target_feature = "avx")]
//     assert!(false);
//     out_line!(4.0);
//     output().flush();
//     exit(0);
// }

fn solve(input: &mut Input, _test_case: usize) {
    {
        // stress();
        // exit(0);
    }
    // check_target_feature();
    let n = input.usize();
    let m = input.usize();
    let q = input.usize();
    let edges = gen_vec(m, |_| Edge {
        fr: input.usize() - 1,
        to: input.usize() - 1,
    });
    let queries = gen_vec(q, |_| {
        let q_type = input.usize();
        if q_type == 1 {
            let edge_id = input.usize() - 1;
            Op::ChangeColor(edge_id)
        } else {
            assert_eq!(q_type, 2);
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            Op::CanReach(fr, to)
        }
    });
    let res = solve_test(n, &edges, &queries, 800);
    for r in res.iter() {
        if !r {
            out_line!("NO");
        } else {
            out_line!("YES");
        }
    }
}

fn stress2() {
    for id in 1.. {
        dbg!(id);
        let mut rnd = Random::new(787788 + id);

        const M: usize = 1000;

        let n = rnd.gen_in_range(1..M);
        let m = rnd.gen_in_range(1..M);
        let edges = gen_vec(m, |_| Edge {
            fr: rnd.gen_in_range(0..n),
            to: rnd.gen_in_range(0..n),
        });
        let queries = gen_vec(m, |_| {
            if rnd.gen_double() < 0.9 {
                Op::CanReach(rnd.gen_in_range(0..n), rnd.gen_in_range(0..n))
            } else {
                Op::ChangeColor(rnd.gen_in_range(0..m))
            }
        });
        let b1 = rnd.gen_in_range(1..M);
        let b2 = rnd.gen_in_range(1..M);

        let v1 = solve_test(n, &edges, &queries, b1);
        let v2 = solve_test(n, &edges, &queries, b2);
        if v1 != v2 {
            dbg!(n);
            dbg!(edges);
            dbg!(v1);
            dbg!(v2);
            dbg!(b1, b2);
            dbg!(queries);
            assert!(false);
        }
    }
}

fn stress() {
    // check_target_feature();
    let n = 50_000;
    let m = 50_000;
    let mut rnd = Random::new(787788);
    let edges = gen_vec(m, |_| Edge {
        fr: rnd.gen_in_range(0..n),
        to: rnd.gen_in_range(0..n),
    });
    let queries = gen_vec(m, |_| {
        if rnd.gen_double() < 0.9 {
            Op::CanReach(rnd.gen_in_range(0..n), rnd.gen_in_range(0..n))
        } else {
            Op::ChangeColor(rnd.gen_in_range(0..m))
        }
    });
    solve_test(n, &edges, &queries, 800);
}

#[derive(Debug)]
enum Op {
    ChangeColor(usize),
    CanReach(usize, usize),
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
    // tester::run_tests();
    tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
