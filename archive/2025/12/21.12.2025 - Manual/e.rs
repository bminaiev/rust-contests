//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;
use algo_lib::seg_trees::treap::{NodeRef, Treap};

#[derive(Clone, Default, Copy)]
struct Node {
    cnt: [i32; 2],
    // res = [max (3x - y), max (3y - x)]
    res: [i32; 2],
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            cnt: [l.cnt[0] + r.cnt[0], l.cnt[1] + r.cnt[1]],
            res: [l.res[0].max(r.res[0]), l.res[1].max(r.res[1])],
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.cnt[0] += update[0];
        node.cnt[1] += update[1];
        node.res[0] += 3 * update[0] - update[1];
        node.res[1] += 3 * update[1] - update[0];
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        current[0] += add[0];
        current[1] += add[1];
    }

    // cnt += update
    type Update = [i32; 2];

    type Context = ();
}

#[derive(Clone, Copy)]
struct Dp {
    node: NodeRef,
    // (x - y) of the first value
    offset: i32,
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let s = input.string();
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            g[fr].push(to);
            g[to].push(fr);
        }
        let mut st = Treap::<Node>::new();
        let mut res = vec![0; n];
        RecursiveFunction2::new(|f, v: usize, p: usize| {}).call(0, 0);
        for &x in res.iter() {
            out.println(x);
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
