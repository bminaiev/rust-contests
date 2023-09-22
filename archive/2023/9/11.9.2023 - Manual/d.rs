//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Query {
    l: usize,
    r: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Value {
    value: i64,
    pos: usize,
}

#[derive(Clone, Copy, Default)]
struct Node {
    max: i64,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            max: l.max.max(r.max),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max += *update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    type Update = i64;

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut vals = vec![];
    for pos in 0..n {
        vals.push(Value { value: a[pos], pos });
    }
    vals.sort();
    vals.reverse();
    let q = input.usize();
    let mut max_q = vec![0; n];
    for _ in 0..q {
        let l = input.usize() - 1;
        let r = input.usize();
        max_q[l].update_max(r);
    }
    let mut queries = vec![];
    let mut max_r = 0;
    for l in 0..n {
        if max_q[l] > max_r {
            max_r = max_q[l];
            queries.push(Query { l, r: max_r });
        }
    }
    let mut result = 0;
    let mut st = SegTree::new(queries.len(), |_| Node { max: 0 });
    for val in vals.into_iter() {
        let l = binary_search_first_true(0..queries.len(), |idx| queries[idx].r > val.pos);
        let r = binary_search_first_true(0..queries.len(), |idx| queries[idx].l > val.pos);
        if l < r {
            let mx = st.get(l..r).max;
            result.update_max(mx * val.value);
            st.update(l..r, 1);
        }
    }
    out_line!(result);
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
