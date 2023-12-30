//{"name":"E. Весёлая жизнь в университете","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1\n1 2\n7\n1 1 2 2 3 3\n6 5 2 3 6 5 6\n13\n1 1 1 2 2 2 3 3 4 5 6 6\n2 2 2 1 4 9 7 2 5 2 1 11 2\n12\n1 1 1 2 2 3 4 4 7 7 6\n11 2 1 11 12 8 5 8 8 5 11 7\n","output":"2\n9\n9\n12\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EVesyolayaZhiznVUniversitete"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::lazy_seg_tree_add_sum::Node;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

#[derive(Clone, Default, Copy, Debug)]
pub struct MaxValNode {
    pub max_val: i64,
    pub pos: usize,
}

impl SegTreeNode for MaxValNode {
    #[allow(unused)]
    fn join_nodes(l: &Self, r: &Self, context: &()) -> Self {
        if l.max_val > r.max_val {
            *l
        } else {
            *r
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_val += *update;
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    type Update = i64;
    type Context = ();
}

pub type SegTreeMax = SegTree<MaxValNode>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut p = vec![0; n];
    for i in 1..n {
        p[i] = input.usize() - 1;
    }
    let mut g = vec![vec![]; n];
    for i in 1..n {
        g[p[i]].push(i);
    }
    let color = input.vec::<usize>(n).sub_from_all(1);
    let mut seen: Vec<Vec<usize>> = vec![vec![]; n];
    let mut same_color: Vec<Vec<usize>> = vec![vec![]; n];
    let mut time = 0;
    let mut tin = vec![0; n];
    let mut tout = vec![0; n];
    let mut enabled = vec![true; n];
    RecursiveFunction2::new(|f, v: usize, _p: usize| {
        if let Some(&x) = seen[color[v]].last() {
            same_color[x].push(v);
            enabled[v] = false;
        }
        tin[v] = time;
        time += 1;
        seen[color[v]].push(v);
        for &to in g[v].iter() {
            f.call(to, v);
        }
        seen[color[v]].pop();
        tout[v] = time;
    })
    .call(0, 0);
    let mut st = SegTreeMax::new(time, |pos| MaxValNode { pos, max_val: 0 });
    for v in 0..n {
        if enabled[v] {
            st.update(tin[v]..tout[v], 1);
        }
    }
    let mut res = 1;
    RecursiveFunction2::new(|f, v: usize, _p: usize| {
        let mut children = vec![];
        for &to in g[v].iter() {
            let mx = st.get(tin[to]..tout[to]).max_val;
            children.push(mx);
        }
        children.sort();
        let mx1 = children.pop().unwrap_or(1);
        let mx2 = children.pop().unwrap_or(1);
        res.update_max(mx1 * mx2);
        st.update(tin[v]..tout[v], -1);
        for x in same_color[v].iter() {
            st.update(tin[*x]..tout[*x], 1);
        }
        for &to in g[v].iter() {
            f.call(to, v);
        }
        // st.update(tin[v]..tout[v], 1);
    })
    .call(0, 0);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e_vesyolaya_zhizn_vuniversitete";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
