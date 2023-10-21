//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::hld::Hld;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

#[derive(Clone, Copy, Default)]
struct Node {
    sum: Mod,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self { sum: l.sum + r.sum }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.sum *= *update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current *= *add;
    }

    type Update = Mod;

    type Context = ();
}

struct Solver {
    hld: Hld,
    vertices_st: SegTree<Node>,
    edges_st: SegTree<Node>,
    res: Mod,
}

impl Solver {
    pub fn new(g: Vec<Vec<usize>>) -> Self {
        let n = g.len();
        // let mut vertices = vec![Mod::ONE; n];
        // let mut edges = vec![Mod::ONE; n];

        let vertices_st = SegTree::new(n, |_| Node { sum: Mod::ONE });
        let edges_st = SegTree::new(n, |_| Node { sum: Mod::ONE });

        // let hld = Hld::new(g.clone(), 0);
        // dbg!(hld.order);

        Self {
            hld: Hld::new(g, 0),
            vertices_st,
            edges_st,
            res: Mod::ZERO,
        }
    }

    fn query(&mut self, u: usize, v: usize, add: bool) -> Mod {
        let two = Mod::new(2);
        let inv_two = Mod::ONE / two;

        for range in self.hld.find_path_segs(u, v) {
            if add {
                // for pos in range {
                //     res += vertices[pos];
                //     res -= edges[pos];
                //     vertices[pos] *= two;
                //     edges[pos] *= two;
                // }
                self.res += self.vertices_st.get(range.clone()).sum;
                self.res -= self.edges_st.get(range.clone()).sum;
                self.vertices_st.update(range.clone(), two);
                self.edges_st.update(range.clone(), two);
            } else {
                // for pos in range {
                //     vertices[pos] *= inv_two;
                //     edges[pos] *= inv_two;
                //     res -= vertices[pos];
                //     res += edges[pos];
                // }
                self.vertices_st.update(range.clone(), inv_two);
                self.edges_st.update(range.clone(), inv_two);
                self.res -= self.vertices_st.get(range.clone()).sum;
                self.res += self.edges_st.get(range.clone()).sum;
            }
        }
        let lca = self.hld.lca(u, v);
        let lca = self.hld.pos_in_order[lca];
        if add {
            // edges[lca] *= inv_two;
            // res += edges[lca];
            self.edges_st.update(lca..lca + 1, inv_two);
            self.res += self.edges_st.get(lca..lca + 1).sum;
        } else {
            // res -= edges[lca];
            // edges[lca] *= two;
            self.res -= self.edges_st.get(lca..lca + 1).sum;
            self.edges_st.update(lca..lca + 1, two);
        }
        self.res
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut solver = Solver::new(g);
    let qq = input.usize();
    for _ in 0..qq {
        let q_type = input.string();
        let add = q_type[0] == b'+';
        let v = input.usize() - 1;
        let u = input.usize() - 1;
        let res = solver.query(u, v, add);
        out_line!(res);
    }
}

fn stress() {
    const MAX_N: usize = 20;

    for it in 4.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..MAX_N);
        let mut parent = vec![0; n];
        for i in 1..n {
            parent[i] = rnd.gen(0..i);
        }
        let mut g = vec![vec![]; n];
        for i in 1..n {
            g[parent[i]].push(i);
            g[i].push(parent[i]);
        }
        // dbg!(g);
        let mut solver = Solver::new(g);
        let qq = rnd.gen(1..MAX_N);
        let mut pairs = vec![];
        for _ in 0..qq {
            let my_res = if pairs.is_empty() || rnd.gen_bool() {
                let u = rnd.gen(0..n);
                let v = rnd.gen(0..n);
                // dbg!("ADD", u, v);
                pairs.push((u, v));
                solver.query(u, v, true)
            } else {
                let pos = rnd.gen(0..pairs.len());
                let (u, v) = pairs[pos];
                // dbg!("REM", u, v);
                pairs.remove(pos);
                solver.query(u, v, false)
            };
            let mut expected = Mod::ZERO;
            let mut g_pairs = vec![vec![false; pairs.len()]; pairs.len()];
            for i in 0..pairs.len() {
                for j in 0..pairs.len() {
                    let mut cnt = vec![0; n];
                    for (u, v) in [pairs[i], pairs[j]].iter() {
                        let mut u = *u;
                        let mut v = *v;
                        while u != v {
                            if u > v {
                                std::mem::swap(&mut u, &mut v);
                            }
                            cnt[v] += 1;
                            v = parent[v];
                        }
                        cnt[u] += 1;
                    }
                    if cnt.iter().any(|x| *x > 1) {
                        g_pairs[i][j] = true;
                    }
                }
            }
            for mask in 1..1 << pairs.len() {
                let mut ok = true;
                for i in 0..pairs.len() {
                    for j in i + 1..pairs.len() {
                        if ((1 << i) & mask) != 0 {
                            if ((1 << j) & mask) != 0 {
                                if !g_pairs[i][j] {
                                    ok = false;
                                }
                            }
                        }
                    }
                }
                if ok {
                    expected += Mod::ONE;
                }
            }
            // dbg!(pairs, my_res, expected);
            assert_eq!(my_res, expected);
        }
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
