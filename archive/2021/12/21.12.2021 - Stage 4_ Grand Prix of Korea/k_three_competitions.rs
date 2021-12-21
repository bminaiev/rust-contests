//{"name":"K. Three Competitions","group":"Yandex - Stage 4: Grand Prix of Korea","url":"https://official.contest.yandex.ru/opencupXXII/contest/30766/problems/K/","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n2 4 3\n3 1 4\n4 3 2\n1 2 1\n3\n1 2\n2 1\n3 4\n","output":"YES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KThreeCompetitions"}}}

use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::graph_builder::GraphBuilder;
use algo_lib::graph::strongly_connected_components::find_strongly_connected_component;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::rand::Random;
use algo_lib::{dbg, out, out_line};
use std::time::Instant;

#[derive(Copy, Clone)]
struct Person {
    p1: usize,
    p2: usize,
    p3: usize,
}

#[derive(Copy, Clone, Debug)]
struct Node {
    left: i32,
    right: i32,
}

const EMPTY_NODE: Node = Node { left: 0, right: 0 };

struct MyGraph {
    nodes: Vec<Node>,
    builder: GraphBuilder<SimpleEdge>,
}

impl MyGraph {
    fn add_vertex(&mut self) -> usize {
        self.nodes.push(EMPTY_NODE);
        self.builder.add_vertex();
        self.nodes.len() - 1
    }
}

fn add_edges(my_node: usize, v: usize, l: usize, r: usize, after: usize, graph: &mut MyGraph) {
    if v == 0 {
        return;
    }
    if l >= after {
        graph.builder.add_edge(my_node, SimpleEdge::new(v));
        return;
    }
    if r <= after {
        return;
    }
    let m = (l + r) >> 1;
    add_edges(my_node, graph.nodes[v].left as usize, l, m, after, graph);
    add_edges(my_node, graph.nodes[v].right as usize, m, r, after, graph);
}

fn add_vertex(
    my_node: usize,
    v: usize,
    l: usize,
    r: usize,
    pos: usize,
    graph: &mut MyGraph,
) -> usize {
    let cloned_v = graph.add_vertex();
    graph.nodes[cloned_v].left = graph.nodes[v].left;
    graph.nodes[cloned_v].right = graph.nodes[v].right;

    if l + 1 == r {
        graph.builder.add_edge(cloned_v, SimpleEdge::new(my_node));
    } else {
        let m = (l + r) >> 1;
        if m > pos {
            graph.nodes[cloned_v].left = add_vertex(
                my_node,
                graph.nodes[cloned_v].left as usize,
                l,
                m,
                pos,
                graph,
            ) as i32;
        } else {
            graph.nodes[cloned_v].right = add_vertex(
                my_node,
                graph.nodes[cloned_v].right as usize,
                m,
                r,
                pos,
                graph,
            ) as i32;
        }
    }

    cloned_v
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let p1 = input.usize() - 1;
        let p2 = input.usize() - 1;
        let p3 = input.usize() - 1;
        a.push(Person { p1, p2, p3 });
    }
    let comps = solve_task(a);
    let tc = input.usize();
    for _ in 0..tc {
        let from = input.usize();
        let to = input.usize();
        let ans = if comps[from] <= comps[to] {
            "YES"
        } else {
            "NO"
        };
        out_line!(ans);
    }
}

fn solve_task(mut a: Vec<Person>) -> Vec<u32> {
    let n = a.len();
    let graph_builder: GraphBuilder<SimpleEdge> = GraphBuilder::new(n + 1);
    let mut my_graph = MyGraph {
        nodes: vec![EMPTY_NODE; n + 1],
        builder: graph_builder,
    };
    for rot in 0..2 {
        let mut who = vec![0; n];
        for (idx, p) in a.iter().enumerate() {
            who[p.p1] = idx;
        }

        let mut roots = vec![0; 2];

        for pos in (0..n).rev() {
            let p_id = who[pos];

            let p = a[p_id];

            for tree_id in 0..(2 - rot) {
                let my_pos = if tree_id == 0 { p.p3 } else { p.p2 };
                add_edges(p_id + 1, roots[tree_id], 0, n, my_pos, &mut my_graph);
                roots[tree_id] = add_vertex(p_id + 1, roots[tree_id], 0, n, my_pos, &mut my_graph);
            }
        }

        for i in 0..n {
            let p1 = a[i].p1;
            let p2 = a[i].p2;
            a[i].p1 = p2;
            a[i].p2 = p1;
        }
    }
    for v in 0..my_graph.nodes.len() {
        if my_graph.nodes[v].left != 0 {
            my_graph
                .builder
                .add_edge(v, SimpleEdge::new(my_graph.nodes[v].left as usize));
        }
        if my_graph.nodes[v].right != 0 {
            my_graph
                .builder
                .add_edge(v, SimpleEdge::new(my_graph.nodes[v].right as usize));
        }
    }
    let real_graph = my_graph.builder.build();
    let comps = find_strongly_connected_component(&real_graph);
    comps
}

fn stress() {
    loop {
        let mut rnd = Random::new(7877881);
        let n = 200_000;
        let p1 = rnd.next_permutation(n);
        let p2 = rnd.next_permutation(n);
        let p3 = rnd.next_permutation(n);
        let a: Vec<_> = (0..n)
            .map(|id| Person {
                p1: p1[id],
                p2: p2[id],
                p3: p3[id],
            })
            .collect();
        let start = Instant::now();
        solve_task(a);
        dbg!(start.elapsed().as_millis());
        break;
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    // stress();
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    // stress();
    tester::run_tests();
}
//END MAIN
