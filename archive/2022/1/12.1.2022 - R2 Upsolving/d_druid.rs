//{"name":"D. Druid","group":"Yandex - R2 Upsolving","url":"https://contest.yandex.ru/contest/34405/problems/D/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2 2\n2 1\n","output":"1 2\n"},{"input":"3\n3 1 2\n2 1\n3 2\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDruid"}}}

use algo_lib::graph::bfs::bfs;
use algo_lib::graph::compressed_graph::CompressedGraph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::{dbg, out, out_line};
use std::collections::BTreeSet;

fn solve_with_root(
    graph: &CompressedGraph<SimpleEdge>,
    root: usize,
    w: &[usize],
) -> Option<Vec<usize>> {
    let n = graph.num_vertices();
    let bfs = bfs(root, graph);
    let mut leaf_by_w = vec![None; n];
    for v in 0..n {
        if leaf_by_w[w[v]].is_none() || bfs.dist[leaf_by_w[w[v]].unwrap()] < bfs.dist[v] {
            leaf_by_w[w[v]] = Some(v);
        }
    }
    let mut res = vec![0; n];
    let mut not_used_nodes = BTreeSet::new();
    let mut seen = vec![false; n];
    for cur_w in (0..n).rev() {
        if let Some(mut v) = leaf_by_w[cur_w] {
            seen[v] = true;
            res[v] = cur_w;
            while v != root {
                let prev = bfs.prev[v];
                if w[prev] < w[v] {
                    return None;
                }
                if w[prev] > w[v] {
                    break;
                }
                v = prev;
                seen[v] = true;
                not_used_nodes.insert(v);
            }
        } else {
            if let Some(&use_node) = not_used_nodes.iter().next_back() {
                not_used_nodes.remove(&use_node);
                res[use_node] = cur_w;
            } else {
                return None;
            }
        }
    }
    if seen.iter().any(|&x| !x) {
        return None;
    }
    Some(res)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let w = input.read_vec::<usize>(n).sub_from_all(1);
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let mut options: Vec<Vec<usize>> = vec![];
    let mut checked = 0;
    for root in 0..n {
        if w[root] == n - 1 {
            let cnt_big = graph
                .adj(root)
                .iter()
                .filter(|edge| w[edge.to()] == n - 1)
                .count();
            if cnt_big <= 1 {
                checked += 1;
                if checked > 2 {
                    break;
                }
                if let Some(check) = solve_with_root(&graph, root, &w) {
                    options.push(check);
                }
            }
        }
    }
    options.sort();
    if options.is_empty() {
        out_line!(-1);
    } else {
        let res = options[0].clone().add_to_all(1);
        out_line!(res);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
