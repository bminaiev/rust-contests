//{"name":"F. Встречи панд","group":"Codeforces - Codeforces Round 887 (Div. 1)","url":"https://codeforces.com/contest/1852/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n0 6 3\n4 2 -5\n7 4 -6\n10 5 100\n10 8 7\n","output":"0\n3\n3\n3\n10\n"},{"input":"5\n0 6 3\n4 2 -5\n7 4 -6\n10 5 100\n11 8 7\n","output":"0\n3\n3\n3\n9\n"},{"input":"7\n0 8 6\n2 7 -2\n3 1 -6\n5 3 -8\n7 3 -3\n8 0 -2\n8 2 1\n","output":"0\n0\n6\n6\n6\n6\n7\n"},{"input":"4\n0 0 -3\n0 0 2\n0 0 4\n0 0 -10\n","output":"0\n2\n3\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FVstrechiPand"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::lazy_seg_tree::SegTreeNode;
use algo_lib::seg_trees::treap::{NodeRef, Treap};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct SegmentEnd {
    t: i32,
    // dp[t + 1] - dp[t]
    dcost: i32,
    // Invariant:
    // dcost > 0 -> dt = +1
    // dcost < 0 -> dt = -1
}

impl SegmentEnd {
    pub fn dt(&self) -> i32 {
        self.dcost.signum()
    }
}

#[derive(Clone, Debug)]
struct Collapse {
    x_pos: i32,
    // idx & idx+1 are collapsed
    idx: u32,
}

#[derive(Clone, Default, Debug)]
struct Node {
    first_collapse: Option<Collapse>,
    min_pref_cost: i32,
    sum_cost: i32,
    last: SegmentEnd,
    first: SegmentEnd,
    cnt_leafs: u32,
}

impl Node {
    pub fn new(t: i32, dcost: i32) -> Self {
        let segm = SegmentEnd { t, dcost };
        Self {
            first_collapse: None,
            min_pref_cost: dcost.min(0),
            sum_cost: dcost,
            last: segm.clone(),
            first: segm,
            cnt_leafs: 1,
        }
    }
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        let mut first_collapse = l.first_collapse.clone();
        if let Some(mut r_collapse) = r.first_collapse.clone() {
            if first_collapse.is_none() || r_collapse.x_pos < first_collapse.as_ref().unwrap().x_pos
            {
                r_collapse.idx += l.cnt_leafs;
                first_collapse = Some(r_collapse);
            }
        }
        {
            let last = &l.last;
            let first = &r.first;
            if last.dt() > 0 && first.dt() < 0 {
                let collapse = Collapse {
                    x_pos: (first.t - last.t + 1) / 2,
                    idx: l.cnt_leafs - 1,
                };
                if first_collapse.is_none()
                    || collapse.x_pos < first_collapse.as_ref().unwrap().x_pos
                {
                    first_collapse = Some(collapse);
                }
            }
        }

        Self {
            first_collapse,
            min_pref_cost: min(l.min_pref_cost, l.sum_cost + r.min_pref_cost),
            sum_cost: l.sum_cost + r.sum_cost,
            last: r.last.clone(),
            first: l.first.clone(),
            cnt_leafs: l.cnt_leafs + r.cnt_leafs,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        if let Some(first_collapse) = &mut node.first_collapse {
            first_collapse.x_pos -= update;
            assert!(first_collapse.x_pos > 0);
        }
        node.first.t += node.first.dt() * *update;
        node.last.t += node.last.dt() * *update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += add;
    }

    // increase x by this amount
    type Update = i32;

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n_queries = input.usize();
    let mut prev_x = 0;
    let mut base_answer = 0;

    let mut tree = Treap::<Node>::new();
    let mut root = NodeRef::NULL;

    for _ in 0..n_queries {
        let x = input.i32();
        let t = input.i32();
        let cnt = input.i32();

        let dx = x - prev_x;

        while let Some(node) = tree.get_node(root) {
            if let Some(first_collapse) = &node.first_collapse {
                if first_collapse.x_pos > dx {
                    break;
                }
                let idx = first_collapse.idx as usize;
                let first = tree.remove(&mut root, idx).clone();
                let second = tree.remove(&mut root, idx).clone();
                let first_dcost = first.last.dcost;
                let second_dcost = second.first.dcost;
                let dcost = first_dcost + second_dcost;
                if dcost > 0 {
                    tree.insert(&mut root, idx, Node::new(first.last.t, dcost));
                } else if dcost < 0 {
                    tree.insert(&mut root, idx, Node::new(second.first.t, dcost));
                }
            } else {
                break;
            }
        }

        let cnt_leafs = tree.len(root);
        tree.update(&mut root, 0..cnt_leafs, &dx);

        let dcost = -cnt;
        if cnt > 0 {
            base_answer += cnt;
        };
        let index = tree.find_first_true_pos(root, |node| node.last.t >= t);

        match tree.query(root, index..index + 1) {
            Some(cur_node) if cur_node.first.t == t => {
                let ndcost = dcost + cur_node.last.dcost;
                tree.remove(&mut root, index);
                if ndcost != 0 {
                    tree.insert(&mut root, index, Node::new(t, ndcost));
                }
            }
            _ => {
                tree.insert(&mut root, index, Node::new(t, dcost));
            }
        }

        prev_x = x;

        let res = match tree.get_node(root) {
            None => 0,
            Some(node) => node.min_pref_cost,
        };

        out_line!(base_answer + res);
    }
    assert!(tree.num_nodes() <= 2 * n_queries + 10);
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
    // tester::run_single_tqest("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
