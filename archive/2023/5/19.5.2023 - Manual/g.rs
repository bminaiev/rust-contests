//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::Range;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree::LazySegTreeNodeSpec;
use algo_lib::seg_trees::persistent_tree::{NodeId, PersistentSegTree, TreeNode};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

#[derive(Clone, Default)]
struct Node {
    hash: Mod,
    cnt: i32,
    len: usize,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self {
            hash: l.hash * context[r.len] + r.hash,
            cnt: l.cnt + r.cnt,
            len: l.len + r.len,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.cnt += update;
        node.hash = Mod::new(node.cnt);
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    type Update = i32;

    type Context = Vec<Mod>;
}

#[derive(Clone)]
struct Way {
    cost: NodeId,
    remote_at: i64,
}

fn print_all(
    st: &PersistentSegTree<Node>,
    node: TreeNode<Node>,
    res: &mut Vec<usize>,
    range: Range<usize>,
) {
    if range.len() == 1 {
        for _ in 0..node.inner().cnt {
            res.push(range.start);
        }
        return;
    } else {
        let (l, r) = st.get_children(&node);
        let m = range.start + (range.end - range.start) / 2;
        print_all(st, l, res, range.start..m);
        print_all(st, r, res, m..range.end);
    }
}

fn solve_slow(positions: &[i64], sizes: &[usize], l: i64, r: i64) -> Option<Vec<usize>> {
    let mut dp = vec![vec![]; positions.len()];
    dp[0] = vec![sizes[0]];
    for v in 0..positions.len() {
        if dp[v].is_empty() {
            continue;
        }
        for u in v + 1..positions.len() {
            if positions[v] + l <= positions[u] && positions[v] + r >= positions[u] {
                let mut new = dp[v].clone();
                new.push(sizes[u]);
                new.sort();
                new.reverse();
                if new > dp[u] {
                    dp[u] = new;
                }
            }
        }
    }
    if dp[positions.len() - 1].is_empty() {
        None
    } else {
        Some(dp[positions.len() - 1].clone())
    }
}

fn solve_st(positions: &[i64], sizes: &[usize], l: i64, r: i64) -> Option<Vec<usize>> {
    let n = positions.len();
    let pows = Mod::gen_powers(Mod::new(239017), n + 1);
    let (mut st, root) = PersistentSegTree::new_f_with_context(
        n,
        &|_| Node {
            hash: Mod::new(0),
            cnt: 0,
            len: 1,
        },
        pows,
    );
    // st.reserve(50 * n);
    let mut add = vec![vec![]; n + 1];
    add[0].push(Way {
        cost: root,
        remote_at: positions[0] + 1,
    });

    let show = |st: &PersistentSegTree<Node>, node: NodeId| {
        let mut res = vec![];
        print_all(st, st.node(node).clone(), &mut res, 1..n + 1);
        dbg!(res);
    };

    let cmp = |st: &PersistentSegTree<Node>, n1: NodeId, n2: NodeId| -> Ordering {
        let mut n1 = st.node(n1).clone();
        let mut n2 = st.node(n2).clone();
        if n1.inner().hash == n2.inner().hash {
            return Ordering::Equal;
        }
        loop {
            if n1.inner().len == 1 {
                return n1.inner().cnt.cmp(&n2.inner().cnt);
            }
            if n1.inner().cnt == 0 {
                return Ordering::Less;
            }
            if n2.inner().cnt == 0 {
                return Ordering::Greater;
            }
            let (l1, r1) = st.get_children(&n1);
            let (l2, r2) = st.get_children(&n2);
            if r1.inner().hash == r2.inner().hash {
                n1 = l1;
                n2 = l2;
            } else {
                n1 = r1;
                n2 = r2;
            }
        }
    };

    // front - best, but could not live long enough
    let mut queue = VecDeque::<Way>::new();
    let mut res = NodeId::NONE;
    let mut first_ok_idx = 0;
    for i in 0..n {
        for w in add[i].iter() {
            while let Some(last) = queue.back() {
                if cmp(&st, last.cost, w.cost) == Ordering::Greater {
                    break;
                } else {
                    queue.pop_back();
                }
            }
            queue.push_back(w.clone());
        }
        while let Some(first) = queue.front() {
            if first.remote_at <= positions[i] {
                queue.pop_front();
            } else {
                break;
            }
        }
        if let Some(first) = queue.front() {
            let root = first.cost;
            let pos = sizes[i];
            assert!(pos < n);
            let ncost = st.update(root, pos..pos + 1, &1);
            if i == n - 1 {
                res = ncost;
            }
            let ok_pos = positions[i] + l;
            while first_ok_idx != n && positions[first_ok_idx] < ok_pos {
                first_ok_idx += 1;
            }
            add[first_ok_idx].push(Way {
                cost: ncost,
                remote_at: positions[i] + r + 1,
            });
        }
    }
    // dbg!(st.len_nodes());
    if res != NodeId::NONE {
        let mut res_v = vec![];
        print_all(&st, st.node(res).clone(), &mut res_v, 0..n);
        res_v.sort();
        res_v.reverse();
        Some(res_v)
    } else {
        None
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let l = input.i64();
    let r = input.i64();
    let positions = input.vec::<i64>(n);
    let sizes = input.vec::<usize>(n).sub_from_all(1);
    if let Some(res) = solve_st(&positions, &sizes, l, r) {
        out_line!(res.len());
        for &x in res.iter() {
            out!(x + 1, "");
        }
        out_line!();
    } else {
        out_line!(-1);
    }
}

fn stress2() {
    for id in 10.. {
        dbg!(id);
        const MAX_N: usize = 10;
        const MAX_LEN: usize = 1000000;
        let mut rnd = Random::new(id);
        let go = rnd.gen_nonempty_range(MAX_LEN);
        let (l, r) = (go.start as i64, go.end as i64);
        if l == 0 {
            continue;
        }
        let mut x = vec![];
        let n = rnd.gen(2..MAX_N);
        for _ in 0..n {
            x.push(rnd.gen(1..MAX_LEN as i64));
        }
        x.sort();
        x.dedup();
        let a = gen_vec(x.len(), |_| rnd.gen(0..x.len()));

        let res_st = solve_st(&x, &a, l, r);
        let res_slow = solve_slow(&x, &a, l, r);
        if res_st != res_slow {
            dbg!(x);
            dbg!(a);
            dbg!(l, r);
            dbg!(res_st);
            dbg!(res_slow);
            assert!(false);
        }
    }
}

fn stress() {
    for id in 10.. {
        dbg!(id);
        // const MAX_N: usize = 10;
        const MAX_LEN: usize = 100000000;
        let mut rnd = Random::new(id);
        let go = rnd.gen_nonempty_range(MAX_LEN);
        let (l, r) = (go.start as i64, go.end as i64);
        if l == 0 {
            continue;
        }
        let mut x = vec![];
        let n = 1_000_000; //rnd.gen(2..MAX_N);
        for _ in 0..n {
            x.push(rnd.gen(1..MAX_LEN as i64));
        }
        x.sort();
        x.dedup();
        let a = gen_vec(x.len(), |_| rnd.gen(0..x.len()));

        let res_st = solve_st(&x, &a, l, r);
        // let res_slow = solve_slow(&x, &a, l, r);
        // if res_st != res_slow {
        //     dbg!(x);
        //     dbg!(a);
        //     dbg!(l, r);
        //     dbg!(res_st);
        //     dbg!(res_slow);
        //     assert!(false);
        // }
    }
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
