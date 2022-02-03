//{"name":"F. Совершенное паросочетание","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/F","interactive":true,"timeLimit":12000,"tests":[{"input":"6\n1 4\n6 1\n3 2\n1 2\n5 1\n1 4\n2\n1 2\n2\n1 3\n2\n1 5\n1 6\n2\n3\n","output":"1\n1 1\n0\n0\n4\n2 1 3\n0\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSovershennoeParosochetanie"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::edge_with_info::EdgeWithInfo;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::graph::trees::heavy_light::{GoDirection, HeavyLight};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
use algo_lib::{dbg, out, out_line};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Default)]
struct Info {
    cnt: i32,
    sum: i64,
}

impl Add for Info {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cnt: self.cnt + rhs.cnt,
            sum: self.sum + rhs.sum,
        }
    }
}

impl Sub for Info {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            cnt: self.cnt - rhs.cnt,
            sum: self.sum - rhs.sum,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct SegNode {
    alive: Info,
    not_alive: Info,
}

impl LazySegTreeNodeSpec for SegNode {
    fn unite(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            alive: l.alive + r.alive,
            not_alive: l.not_alive + r.not_alive,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        if *update {
            let tmp = node.alive;
            node.alive = node.not_alive;
            node.not_alive = tmp;
        }
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current ^= *add;
    }

    type Update = bool;
    type Context = ();
}

struct HeavyLightNode {
    seg_tree: LazySegTree<SegNode>,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut graph = SimpleGraphT::new(n);
    for id in 1..n {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        graph.add_complex_bi_edge(fr, EdgeWithInfo::new(to, id));
    }
    let mut parent_edge_id = vec![0; n];
    RecursiveFunction2::new(|f, v, edge_id| {
        parent_edge_id[v] = edge_id;
        for edge in graph.adj(v) {
            if edge.info != edge_id {
                f.call(edge.to(), edge.info);
            }
        }
    })
    .call(0, 0);
    let mut alive_vertices = vec![false; n];
    alive_vertices[0] = true;
    let mut total_vertices = 1;
    let mut cur_info = Info { cnt: 0, sum: 0 };

    let mut heavy_light: HeavyLight<HeavyLightNode> =
        HeavyLight::new(&graph, 0, |ids: &[usize]| -> HeavyLightNode {
            let seg_tree = LazySegTree::new_f(ids.len() - 1, &|pos| {
                let v = ids[pos + 1];
                SegNode {
                    alive: Info { cnt: 0, sum: 0 },
                    not_alive: Info {
                        cnt: 1,
                        sum: parent_edge_id[v] as i64,
                    },
                }
            });
            HeavyLightNode { seg_tree }
        });

    loop {
        let query_type = input.usize();
        if query_type == 1 {
            let v = input.usize() - 1;
            alive_vertices[v] = true;
            total_vertices += 1;

            heavy_light.go_path(v, 0, |subpath, range, dir| {
                assert_eq!(dir, GoDirection::RightToLeft);
                {
                    cur_info = cur_info - subpath.extra.seg_tree.get(range.clone()).alive;
                    subpath.extra.seg_tree.update(range.clone(), true);
                    cur_info = cur_info + subpath.extra.seg_tree.get(range.clone()).alive;
                }
            });

            if cur_info.cnt * 2 == total_vertices {
                out_line!(cur_info.sum);
            } else {
                out_line!(0);
            }
        } else if query_type == 2 {
            if cur_info.cnt * 2 == total_vertices {
                let mut edges = vec![];
                RecursiveFunction2::new(|f, v: usize, p| -> usize {
                    if !alive_vertices[v] {
                        return 0;
                    }
                    let mut size = 1;
                    for e in graph.adj(v) {
                        if e.to() == p {
                            continue;
                        }
                        size ^= f.call(e.to(), v);
                    }
                    if size == 1 {
                        edges.push(parent_edge_id[v] as i64);
                    }
                    size
                })
                .call(0, 0);
                assert_eq!(cur_info.sum, edges.iter().sum());
                edges.sort();
                out_line!(edges.len());
                out_line!(edges);
            } else {
                out_line!(0);
            }
        } else {
            assert_eq!(query_type, 3);
            break;
        }
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
    // tester::run_single_test("1");
}
//END MAIN
