//{"name":"D - Roadway","group":"AtCoder - AtCoder Regular Contest 196 (Div. 1)","url":"https://atcoder.jp/contests/arc196/tasks/arc196_d","interactive":false,"timeLimit":3000,"tests":[{"input":"5 4 2\n4 2\n1 3\n3 5\n2 4\n1 3\n2 4\n","output":"Yes\nNo\n"},{"input":"7 6 3\n1 5\n2 4\n4 6\n7 1\n5 3\n1 6\n1 6\n4 4\n2 5\n","output":"No\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRoadway"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};
use algo_lib::seg_trees::persistent_tree::{NodeId, PersistentSegTree};
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct Node {
    xor: u64,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Node { xor: l.xor ^ r.xor }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.xor ^= *update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current ^= *add;
    }

    type Update = u64;

    type Context = ();
}

type PST = PersistentSegTree<Node>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let q = input.usize();
    let mut start = vec![0; m];
    let mut finish = vec![0; m];
    for i in 0..m {
        start[i] = input.usize() - 1;
        finish[i] = input.usize() - 1;
    }
    let mut next_bad = vec![0; m];
    let mut cant_start = vec![m; n];
    let mut cant_finish = vec![m; n];

    let (mut pst_left_right, pst_left_right_root) =
        PersistentSegTree::new_f_with_context(n, &|_| Node { xor: 0 }, ());
    let mut pst_lr_roots = vec![NodeId::NONE; m + 1];
    pst_lr_roots[m] = pst_left_right_root;

    let (mut pst_right_left, pst_right_left_root) =
        PersistentSegTree::new_f_with_context(n, &|_| Node { xor: 0 }, ());
    let mut pst_rl_roots = vec![NodeId::NONE; m + 1];
    pst_rl_roots[m] = pst_right_left_root;

    const BLOCK: usize = 500;
    let mut rnd = Random::new(234234);
    for i in (0..m).rev() {
        let xor = rnd.gen_u64();
        if start[i] < finish[i] {
            next_bad[i] = cant_start[start[i]].min(cant_finish[finish[i]]);

            {
                let cur_xor = pst_left_right
                    .get(pst_lr_roots[i + 1], start[i]..finish[i])
                    .xor;
                let first_change = {
                    let mut ll = i + 1;
                    let mut rr = next_bad[i] + 1;
                    while rr - ll > BLOCK {
                        let mid = (ll + rr) >> 1;
                        let now_xor = pst_left_right
                            .get(pst_lr_roots[mid], start[i]..finish[i])
                            .xor;
                        if now_xor == cur_xor {
                            ll = mid;
                        } else {
                            rr = mid;
                        }
                    }
                    ll - 1
                };

                // binary_search_first_true(i + 1..next_bad[i] + 1, |check_pos| {
                //     let now_xor = pst_left_right
                //         .get(pst_lr_roots[check_pos], start[i]..finish[i] - 1)
                //         .xor;
                //     now_xor != cur_xor
                // });
                // let first_bad = first_change - 1;
                // next_bad[i] = next_bad[i].min(first_bad);
                for j in first_change.max(i + 1)..next_bad[i].min(first_change + BLOCK + 10) {
                    let mut bad = false;
                    if start[j] < start[i] && finish[j] > start[i] && finish[j] < finish[i] {
                        bad = true;
                    }
                    if finish[j] > finish[i] && start[j] > start[i] && start[j] < finish[i] {
                        bad = true;
                    }
                    if bad {
                        next_bad[i] = j;
                        break;
                    }
                }
            }

            cant_start[start[i]] = i;
            cant_finish[finish[i]] = i;
            let last_id = pst_lr_roots[i + 1];
            let last_id = pst_left_right.update(last_id, start[i]..start[i] + 1, &xor);
            let last_id = pst_left_right.update(last_id, finish[i] - 1..finish[i], &xor);
            pst_lr_roots[i] = last_id;
            pst_rl_roots[i] = pst_rl_roots[i + 1];
        } else {
            next_bad[i] = cant_finish[start[i]].min(cant_start[finish[i]]);

            {
                let cur_xor = pst_right_left
                    .get(pst_rl_roots[i + 1], finish[i]..start[i])
                    .xor;
                let first_change = {
                    let mut ll = i + 1;
                    let mut rr = next_bad[i] + 1;
                    while rr - ll > BLOCK {
                        let mid = (ll + rr) >> 1;
                        let now_xor = pst_right_left
                            .get(pst_rl_roots[mid], finish[i]..start[i])
                            .xor;
                        if now_xor == cur_xor {
                            ll = mid;
                        } else {
                            rr = mid;
                        }
                    }
                    ll - 1
                };
                //         binary_search_first_true(i + 1..next_bad[i] + 1, |check_pos| {
                //         let now_xor = pst_right_left
                //             .get(pst_rl_roots[check_pos], finish[i]..start[i] - 1)
                //             .xor;
                //         now_xor != cur_xor
                //     });
                //     let first_bad = first_change - 1;
                //     next_bad[i] = next_bad[i].min(first_bad);
                // }
                for j in first_change.max(i + 1)..next_bad[i].min(first_change + BLOCK + 10) {
                    let mut bad = false;
                    if start[j] > start[i] && finish[j] < start[i] && finish[j] > finish[i] {
                        bad = true;
                    }
                    if finish[j] < finish[i] && start[j] < start[i] && start[j] > finish[i] {
                        bad = true;
                    }
                    if bad {
                        next_bad[i] = j;
                        break;
                    }
                }
            }

            cant_start[finish[i]] = i;
            cant_finish[start[i]] = i;

            let last_id = pst_rl_roots[i + 1];
            let last_id = pst_right_left.update(last_id, start[i] - 1..start[i], &xor);
            let last_id = pst_right_left.update(last_id, finish[i]..finish[i] + 1, &xor);
            pst_rl_roots[i] = last_id;
            pst_lr_roots[i] = pst_lr_roots[i + 1];
        }
    }
    let mut st = SegTreeMax::new(m, |pos| MaxValNode {
        max_val: m - next_bad[pos],
        pos,
    });
    for _ in 0..q {
        let left = input.usize() - 1;
        let right = input.usize();

        let next_bad = m - st.get(left..right).max_val;

        if next_bad < right {
            out.println("No");
        } else {
            out.println("Yes");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_roadway";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
