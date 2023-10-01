//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use std::cmp::Reverse;

use algo_lib::graph::dfs_order::DfsOrder;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::hld::Hld;
use algo_lib::seg_trees::seg_tree_2d::SegTree2d;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Color {
    min_v: usize,
    max_v: usize,
    dist: usize,
}

#[derive(Clone, Copy, Default)]
struct Node {
    value: i32,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self {
            value: l.value.max(r.value),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        todo!()
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        todo!()
    }

    type Update = ();

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let colors = input.vec::<usize>(n).sub_from_all(1);
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let u = input.usize() - 1;
        let v = input.usize() - 1;
        g[u].push(v);
        g[v].push(u);
    }
    let hld = Hld::new(g.clone(), 0);
    let dfs_order = DfsOrder::new(&g, 0);
    let mut by_color = vec![vec![]; n];
    for v in 0..n {
        by_color[colors[v]].push(v);
    }
    let mut pairs = vec![];
    for color in 0..colors.len() {
        if by_color[color].len() == 2 {
            let (v, u) = (by_color[color][0], by_color[color][1]);
            let lca = hld.lca(v, u);
            let dist = dfs_order.info[v].height + dfs_order.info[u].height
                - 2 * dfs_order.info[lca].height;
            let pos1 = dfs_order.info[v].pos;
            let pos2 = dfs_order.info[u].pos;
            let (min_v, max_v) = if pos1 < pos2 { (v, u) } else { (u, v) };
            pairs.push(Color { min_v, max_v, dist })
        }
    }
    let mut st_pts: Vec<_> = pairs
        .iter()
        .map(|p| (dfs_order.info[p.min_v].pos, dfs_order.info[p.max_v].pos))
        .collect();
    st_pts.push((0, 0));
    let mut st = SegTree2d::<usize, Node>::new(st_pts);
    pairs.sort_by_key(|p| Reverse(p.dist));
    let mut glob_res = 1;
    for pair in pairs.iter() {
        let (min_v, max_v) = (pair.min_v, pair.max_v);
        let mut res;
        if dfs_order.is_in_subtree_of(max_v, min_v) {
            res = st
                .query(0..dfs_order.info[min_v].pos, dfs_order.info[max_v].range())
                .value;
            res.update_max(
                st.query(
                    dfs_order.info[max_v].range(),
                    dfs_order.info[min_v].max_subtree_pos..n,
                )
                .value,
            );
            for &to in g[min_v].iter() {
                if !dfs_order.is_in_subtree_of(max_v, to) && dfs_order.info[min_v].parent != to {
                    let r1 = dfs_order.info[max_v].range();
                    let r2 = dfs_order.info[to].range();
                    if r1.start < r2.start {
                        res.update_max(st.query(r1, r2).value);
                    } else {
                        res.update_max(st.query(r2, r1).value);
                    }
                }
            }
        } else {
            res = st
                .query(dfs_order.info[min_v].range(), dfs_order.info[max_v].range())
                .value;
        }
        let mut update_res = res + 2;
        st.update(
            dfs_order.info[min_v].pos,
            dfs_order.info[max_v].pos,
            Node { value: update_res },
        );
        if dfs_order.info[max_v].parent != min_v {
            update_res += 1;
        }
        glob_res.update_max(update_res);
    }
    out_line!(glob_res);
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
