#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

#[derive(Clone, Copy, Default)]
struct Node {
    min: i64,
}

impl SegTreeNode for Node {
    type Update = i64;
    type Context = ();

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.min += *update;
    }

    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Node {
            min: l.min.min(r.min),
        }
    }
}

type ST = SegTree<Node>;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n).sub_from_all(1);
        let mut where_located = vec![0; n];
        for i in 0..n {
            where_located[a[i]] = i;
        }
        let end = 2 * n;
        let mut st = ST::new(n * 2, |i| Node { min: -(i as i64) });
        for i in 0..n - 1 {
            let pos1 = where_located[i];
            let pos2 = where_located[i + 1];
            let max_pos = pos1.max(pos2);
            st.update(max_pos..end, 1);
        }
        let mut res = 0;
        for start in 0..n {
            let root_min = st.get(start..start + n);
            if root_min.min > -2 - start as i64 {
                res += 1;
            }
            let cur_value = a[start];
            if cur_value > 0 {
                let pos2 = where_located[cur_value - 1];
                st.update(pos2..end, -1);
                st.update(start + n..end, 1);
            }
            if cur_value + 1 < n {
                let pos2 = where_located[cur_value + 1];
                st.update(pos2..end, -1);
                st.update(start + n..end, 1);
            }
            where_located[cur_value] = start + n;
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "task_13947";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
