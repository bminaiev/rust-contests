//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Item {
    delta: i64,
    id: usize,
    time: usize,
    x: i64,
}

#[derive(Debug, Clone, Default)]
struct Node {
    cnt_alive: usize,
    dp_pairs: [i64; 2],
    dp_ones: i64,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        let dp_pairs = match l.cnt_alive % 4 {
            0 => [l.dp_pairs[0] + r.dp_pairs[0], l.dp_pairs[1] + r.dp_pairs[1]],
            1 => [l.dp_pairs[0] + r.dp_pairs[1], l.dp_pairs[1] - r.dp_pairs[0]],
            2 => [l.dp_pairs[0] - r.dp_pairs[0], l.dp_pairs[1] - r.dp_pairs[1]],
            3 => [l.dp_pairs[0] - r.dp_pairs[1], l.dp_pairs[1] + r.dp_pairs[0]],
            _ => unreachable!(),
        };

        Self {
            cnt_alive: l.cnt_alive + r.cnt_alive,
            dp_ones: l.dp_ones + (if l.cnt_alive % 2 == 0 { 1 } else { -1 }) * r.dp_ones,
            dp_pairs,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        let (alive, delta) = *update;
        node.cnt_alive = if alive { 1 } else { 0 };
        node.dp_ones = delta;
        node.dp_pairs[0] = delta;
        node.dp_pairs[1] = delta;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = (bool, i64);

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut items = vec![];
    for id in 0..n {
        let x = input.i64();
        let y = input.i64();
        items.push(Item {
            delta: y - x,
            id,
            time: 0,
            x,
        })
    }
    for time in 1..=m {
        let id = input.usize() - 1;
        let x = input.i64();
        let y = input.i64();
        items.push(Item {
            delta: y - x,
            id,
            time,
            x,
        })
    }
    items.sort();
    items.reverse();
    let mut cur_item_id = vec![0; n];
    let mut events = vec![0; m];
    let mut sum_x = 0;
    let mut sum_y = 0;
    for i in 0..items.len() {
        if items[i].time == 0 {
            cur_item_id[items[i].id] = i;
            sum_x += items[i].x;
            sum_y += items[i].x + items[i].delta;
        } else {
            events[items[i].time - 1] = i;
        }
    }
    let mut cnt_good = 0;
    for i in 0..n {
        if items[cur_item_id[i]].delta >= 0 {
            cnt_good += 1;
        }
    }
    let mut seg_tree = LazySegTree::new(&Node::default(), items.len() + 1, ());
    for i in 0..n {
        let pos = cur_item_id[i];
        seg_tree.update(pos..pos + 1, (true, items[pos].delta));
    }
    let mut bad_from = 0;
    while bad_from != items.len() && items[bad_from].delta >= 0 {
        bad_from += 1;
    }
    let calc_bob_delta = |seg_tree: &mut LazySegTree<Node>, cnt_good: i32| {
        let node_good = seg_tree.get(0..bad_from);
        let node_bad = seg_tree.get(bad_from..seg_tree.len());
        let mut res = node_good.dp_pairs[1];
        if cnt_good % 2 == 0 {
            res += node_bad.dp_ones;
        } else {
            res -= node_bad.dp_ones;
        }
        res
    };
    {
        let bob_delta = calc_bob_delta(&mut seg_tree, cnt_good);
        let start_res = (sum_x + sum_y - bob_delta) / 2;
        out_line!(start_res);
    }
    for time in 0..m {
        let pos = events[time];
        let id = items[pos].id;
        let prev = cur_item_id[id];
        if items[prev].delta >= 0 {
            cnt_good -= 1;
        }
        seg_tree.update(prev..prev + 1, (false, 0));
        cur_item_id[id] = pos;
        seg_tree.update(pos..pos + 1, (true, items[pos].delta));
        if items[pos].delta >= 0 {
            cnt_good += 1;
        }
        sum_x -= items[prev].x;
        sum_y -= items[prev].x + items[prev].delta;

        sum_x += items[pos].x;
        sum_y += items[pos].x + items[pos].delta;
        {
            let bob_delta = calc_bob_delta(&mut seg_tree, cnt_good);
            let res = (sum_x + sum_y - bob_delta) / 2;
            out_line!(res);
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
