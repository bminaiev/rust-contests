//{"name":"F. Vacation","group":"Yandex - Stage 19: Grand Prix of China","url":"https://official.contest.yandex.com/opencupXXII/contest/39025/problems/F/","interactive":false,"timeLimit":4000,"tests":[{"input":"5 6 3\n0 -5 -3 8 -3\n2 3 5\n1 2 5\n2 1 5\n1 4 -3\n2 3 5\n2 1 5\n","output":"8\n10\n0\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FVacation"}}}

use std::cmp::{max, min};
use std::result;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Default)]
struct Node {
    max_a: i64,
    max_b: i64,
    max_res: i64,
}

#[derive(Clone, Copy)]
struct Update {
    delta_a: i64,
    delta_b: i64,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            max_a: max(l.max_a, r.max_a),
            max_b: max(l.max_b, r.max_b),
            max_res: max(max(l.max_res, r.max_res), r.max_a + l.max_b),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_a += update.delta_a;
        node.max_b += update.delta_b;
        node.max_res += update.delta_a + update.delta_b;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        current.delta_a += add.delta_a;
        current.delta_b += add.delta_b;
    }

    type Update = Update;

    type Context = ();
}

type SegTree = LazySegTree<Node>;

#[derive(Clone, Copy, Default)]
struct MaxSubSegmNode {
    max_pref: i64,
    max_suf: i64,
    sum: i64,
    max_res: i64,
}

impl LazySegTreeNodeSpec for MaxSubSegmNode {
    fn unite(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            max_pref: max(l.max_pref, r.max_pref + l.sum),
            max_suf: max(r.max_suf, l.max_suf + r.sum),
            sum: l.sum + r.sum,
            max_res: max(max(l.max_res, r.max_res), l.max_suf + r.max_pref),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_pref += update;
        node.max_suf += update;
        node.max_res += update;
        node.sum += update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += add;
    }

    type Update = i64;

    type Context = ();
}

fn stupid(a: &[i64], block_size: usize) -> i64 {
    let mut res = 0;
    for i in 0..a.len() {
        let mut sum = 0;
        for j in i + 1..=min(a.len(), i + block_size) {
            sum += a[j - 1];
            res.update_max(sum);
        }
    }
    return res;
}

#[derive(Clone, Copy)]
enum Query {
    Change(usize, i64),
    Query(usize, usize),
}

fn fast(a: &[i64], block_size: usize, queries: &[Query]) -> Vec<i64> {
    let mut a = a.to_vec();
    let n = a.len();
    let mut seg_trees =
        vec![SegTree::new(&Node::default(), block_size + 1, ()); n / block_size + 2];
    let mut sub_segm_tree = LazySegTree::new(&MaxSubSegmNode::default(), n, ());
    let mut max_of_seg_trees =
        SegTreeMax::new(&MaxValNode { max_val: 0, pos: 0 }, seg_trees.len(), ());
    let mut max_of_seg_trees_inside =
        SegTreeMax::new(&MaxValNode { max_val: 0, pos: 0 }, seg_trees.len(), ());
    let change_value =
        |pos: usize,
         delta: i64,
         sub_segm_tree: &mut LazySegTree<MaxSubSegmNode>,
         seg_trees: &mut Vec<LazySegTree<Node>>,
         max_of_seg_trees: &mut LazySegTree<MaxValNode<i64>>,
         max_of_seg_trees_inside: &mut LazySegTree<MaxValNode<i64>>| {
            let prev_seg_tree = pos / block_size;
            let relative_pos = pos % block_size;
            seg_trees[prev_seg_tree].update(
                1 + relative_pos..block_size + 1,
                Update {
                    delta_a: 0,
                    delta_b: delta,
                },
            );
            let nmax = seg_trees[prev_seg_tree].get(0..block_size).max_res;
            max_of_seg_trees.update(prev_seg_tree..prev_seg_tree + 1, nmax);
            seg_trees[prev_seg_tree + 1].update(
                0..relative_pos + 1,
                Update {
                    delta_a: delta,
                    delta_b: 0,
                },
            );
            let nmax = seg_trees[prev_seg_tree + 1].get(0..block_size).max_res;
            max_of_seg_trees.update(prev_seg_tree + 1..prev_seg_tree + 2, nmax);
            sub_segm_tree.update(pos..pos + 1, delta);
            let cur_segment_max = sub_segm_tree
                .get(block_size * prev_seg_tree..block_size * (prev_seg_tree + 1))
                .max_res;
            max_of_seg_trees_inside.update(prev_seg_tree..prev_seg_tree + 1, cur_segment_max);
        };
    for pos in 0..a.len() {
        change_value(
            pos,
            a[pos],
            &mut sub_segm_tree,
            &mut seg_trees,
            &mut max_of_seg_trees,
            &mut max_of_seg_trees_inside,
        );
    }
    let mut results = vec![];
    for query in queries.iter() {
        if let &Query::Change(pos, value) = query {
            let delta = value - a[pos];
            a[pos] = value;
            change_value(
                pos,
                delta,
                &mut sub_segm_tree,
                &mut seg_trees,
                &mut max_of_seg_trees,
                &mut max_of_seg_trees_inside,
            );
        } else if let &Query::Query(l, r) = query {
            let mut res = 0;
            if r - l <= block_size {
                res.update_max(sub_segm_tree.get(l..r).max_res);
            } else {
                let seg_tree_id = l / block_size + 1;
                let relative_pos = l % block_size;
                let len = min(block_size - relative_pos, r - block_size - l);
                res.update_max(
                    seg_trees[seg_tree_id]
                        .get(relative_pos..relative_pos + len)
                        .max_res,
                );
                res.update_max(sub_segm_tree.get(r - block_size..r).max_res);
                res.update_max(sub_segm_tree.get(l..l + block_size).max_res);
                let last_seg_tree_id = (r - 1) / block_size;
                if seg_tree_id + 1 < last_seg_tree_id {
                    res.update_max(
                        max_of_seg_trees
                            .get(seg_tree_id + 1..last_seg_tree_id)
                            .max_val,
                    );
                }
                if seg_tree_id < last_seg_tree_id {
                    res.update_max(
                        max_of_seg_trees_inside
                            .get(seg_tree_id..last_seg_tree_id)
                            .max_val,
                    );
                }
                if last_seg_tree_id > seg_tree_id {
                    let more = r - last_seg_tree_id * block_size;
                    res.update_max(seg_trees[last_seg_tree_id].get(0..more + 1).max_res);
                }
            }
            results.push(res);
        } else {
            assert!(false);
        }
    }
    results
}

fn stress() {
    const N: usize = 100;
    const MAX_V: i64 = 100;
    for it in 8115.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..N);
        let a = gen_vec(n, |_| rnd.gen(-MAX_V..MAX_V));
        let range = rnd.gen_nonempty_range(n);
        let block_size = rnd.gen(1..N);
        let slow = stupid(&a[range.clone()], block_size);
        let fast = fast(&a, block_size, &vec![Query::Query(range.start, range.end)]);
        if slow != fast[0] {
            dbg!(a);
            dbg!(range);
            dbg!(slow);
            dbg!(fast);
            dbg!(block_size);
            assert!(false);
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let block_size = input.usize();
    let a = input.vec::<i64>(n);
    let queries = gen_vec(m, |_| {
        let q_type = input.usize();
        if q_type == 1 {
            Query::Change(input.usize() - 1, input.i64())
        } else {
            assert_eq!(q_type, 2);
            Query::Query(input.usize() - 1, input.usize())
        }
    });
    let res = fast(&a, block_size, &queries);
    out_line!(res);
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
    tester::run_tests();
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_stress(stress);
}
//END MAIN
