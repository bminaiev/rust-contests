use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::seg_trees::bottom_up_seg_tree::BottomUpSegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

const N: usize = 8;
const XORS: [usize; 8] = [0, 3, 6, 9, 12, 15, 5, 10];

#[derive(Clone, Copy, Debug, Default)]
struct Node {
    r: [[i32; N]; N],
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        let mut res = Node::default();
        for i in 0..N {
            for j in 0..N {
                let mut tmp = 0;
                for k in 0..N {
                    tmp = tmp.max(l.r[i][k] + r.r[k][j]);
                }
                res.r[i][j] = tmp;
            }
        }
        res
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

fn solve(input: &mut Input, out: &mut Output) {
    let mut vals = vec![];
    for i in 0..16 {
        let mut a = Node::default();
        for j in 0..N {
            for k in 0..N {
                let real_value = (XORS[j]) ^ (XORS[k]) ^ i;
                if real_value % 3 == 0 {
                    a.r[j][k] = 1;
                }
            }
        }
        vals.push(a);
    }
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let q = input.usize();
        let a = input.vec::<usize>(n);
        let mut st = BottomUpSegTree::new(n, |pos| vals[a[pos]]);
        let mut answers = vec![];
        let r = st.get(0..n).r[0][0];
        answers.push(r);
        for _ in 0..q {
            let pos = input.usize() - 1;
            let value = input.usize();
            st.update_point(pos, vals[value]);
            let r = st.get(0..n).r[0][0];
            answers.push(r);
        }
        out.println(answers);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}
fn stress() {
    let mut a = vec![3, 6, 9, 12, 15];
    for _ in 0..100 {
        let mut more = BTreeSet::new();
        for &x in a.iter() {
            for &y in a.iter() {
                let val = x ^ y;
                if !a.contains(&val) {
                    more.insert(val);
                }
            }
        }
        for x in more {
            a.push(x);
        }
    }
    dbg!(a);
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_sausage_";
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
