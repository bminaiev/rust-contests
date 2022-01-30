//{"name":"E1. Дистанционное дерево (простая версия)","group":"Codeforces - Codeforces Round #769 (Div. 2)","url":"https://codeforces.com/contest/1632/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n1 2\n2 3\n1 4\n2\n1 2\n7\n1 2\n1 3\n3 4\n3 5\n3 6\n5 7\n","output":"1 2 2 2\n1 1\n2 2 3 3 3 3 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1DistantsionnoeDerevoProstayaVersiya"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::graph::bfs::bfs;
use algo_lib::graph::dfs_builder::{dfs_builder, DfsBuilder};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};
use algo_lib::{dbg, out, out_line};
use std::cmp::{max, min};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let bfs_state = bfs(0, &graph);
    let leaf = (0..n)
        .into_iter()
        .max_by_key(|&v| bfs_state.dist[v])
        .unwrap();
    let chain = bfs_state.get_path(leaf).unwrap();
    let mut d = vec![0; chain.len()];
    let mut seen = vec![false; n];
    for &v in chain.iter() {
        seen[v] = true;
    }
    for (pos, &root) in chain.iter().enumerate() {
        d[pos] = RecursiveFunction::new(|f, v| -> usize {
            let mut res = 0;
            seen[v] = true;
            for e in graph.adj(v) {
                if !seen[e.to()] {
                    res.update_max(1 + f.call(e.to()));
                }
            }
            res
        })
            .call(root);
    }
    let mut pref_max = vec![0];
    for (pos, &d) in d.iter().enumerate() {
        let add = max(*pref_max.last_exn(), pos + d);
        pref_max.push(add);
    }

    let mut seg_tree = SegTreeMax::new_f(
        chain.len(),
        &|pos| MaxValNode {
            pos,
            max_val: (d[pos] as i32 - pos as i32),
        },
        (),
    );

    let mut is_ok = |ans: usize, x: usize| -> bool {
        let chain_len = chain.len() - 1;
        if chain_len <= ans {
            return true;
        }
        if x > ans {
            return false;
        }
        let pos = chain_len - (ans - x);

        let first_not_covered = binary_search_first_true(0..pos + 1, |pos| pref_max[pos + 1] > ans);
        if first_not_covered > pos {
            return true;
        }

        let max = seg_tree.get(first_not_covered, pos + 1);

        let len = (x + pos) as i32 + max.max_val;
        if len > ans as i32 {
            return false;
        }
        true
    };

    let mut ans = 0;
    for x in 1..=n {
        while !is_ok(ans, x) {
            ans += 1;
        }
        out!(ans, "");
    }
    out_line!();
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_locally();
}
//END MAIN
