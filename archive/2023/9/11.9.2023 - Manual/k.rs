//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Copy, Clone, Debug, Default)]
struct Node {
    val: f64,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self {
            val: l.val.max(r.val),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.val = *update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = f64;

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = input.vec::<f64>(n);
    let mut b = input.vec::<f64>(n);
    let mut sa = SegTree::new(n, |i| Node { val: a[i] });
    let mut sb = SegTree::new(n, |i| Node { val: b[i] });
    let calc = |a: &[f64], b: &[f64], sa: &mut SegTree<Node>, sb: &mut SegTree<Node>| -> f64 {
        let pos = binary_search_first_true(0..n, |pos| {
            sa.get(0..pos + 1).val + sb.get(0..pos + 1).val >= 1.0
        });
        let max_alice = if pos == 0 { 0.0 } else { sa.get(0..pos).val };
        let max_bob = if pos == 0 { 0.0 } else { sb.get(0..pos).val };
        if a[pos] >= 1.0 - max_bob {
            return 1.0 - max_bob;
        }
        max_alice.max(a[pos])
    };
    out_line!(calc(&a, &b, &mut sa, &mut sb));
    for _ in 0..m {
        let who = input.string();
        let pos = input.usize() - 1;
        let val = input.f64();
        if who[0] == b'A' {
            a[pos] = val;
            sa.update_point(pos, Node { val });
        } else {
            b[pos] = val;
            sb.update_point(pos, Node { val });
        }
        out_line!(calc(&a, &b, &mut sa, &mut sb));
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
