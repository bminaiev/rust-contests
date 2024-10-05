//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod9;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

type Mod = Mod9;

pub struct Context {
    powers: Vec<Mod>,
    #[allow(unused)]
    multiplier: Mod,
}

impl Context {
    pub fn new(max_len: usize, multiplier: Mod) -> Self {
        let mut powers = Vec::with_capacity(max_len + 1);
        powers.push(Mod::ONE);
        for i in 1..=max_len {
            powers.push(powers[i - 1] * multiplier);
        }
        Self { powers, multiplier }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Node {
    hash: Mod,
    hash_rev: Mod,
    size: usize,
}

impl Node {
    pub fn new(hash: Mod) -> Self {
        Self {
            hash,
            hash_rev: hash,
            size: 1,
        }
    }
}

impl SegTreeNode for Node {
    fn join_nodes(lhs: &Self, rhs: &Self, ctx: &Self::Context) -> Self {
        Self {
            hash: lhs.hash * ctx.powers[rhs.size] + rhs.hash,
            hash_rev: rhs.hash_rev * ctx.powers[lhs.size] + lhs.hash_rev,
            size: lhs.size + rhs.size,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        todo!()
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        todo!()
    }

    type Update = ();

    type Context = Context;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let k = Mod::new(input.i32());
    let b = Mod::new(input.i32());
    let a = input.vec::<i32>(n);
    let mut am = vec![Mod::ZERO; n];
    for i in 0..n {
        am[i] = Mod::new(a[i]);
    }
    let mut deltas = vec![Mod::ZERO; n - 1];
    for i in 0..n - 1 {
        deltas[i] = Mod::new(a[i + 1] - a[i]);
    }
    let multiplier = Mod::new(2390171);
    let context = Context::new(n + 1, multiplier);
    let mut deltas_seg_tree = SegTree::new_with_context(
        n,
        |i| {
            if i == n - 1 {
                Node::new(Mod::ZERO)
            } else {
                Node::new(deltas[i])
            }
        },
        context,
    );
    let mut expected_hashes = vec![Mod::ZERO; n + 1];
    for len in 1..=n {
        expected_hashes[len] = expected_hashes[len - 1] * multiplier + k;
    }

    for _ in 0..q {
        let q_type = input.usize();
        if q_type == 1 {
            let l = input.usize() - 1;
            let r = input.usize() - 1;
            let v = Mod::new(input.i32());
            // for i in l..=r {
            //     am[i] += v;
            // }
            if l != 0 {
                deltas[l - 1] += v;
                deltas_seg_tree.update_point(l - 1, Node::new(deltas[l - 1]));
            }
            if r != n - 1 {
                deltas[r] -= v;
                deltas_seg_tree.update_point(r, Node::new(deltas[r]));
            }
        } else {
            assert_eq!(q_type, 2);
            let mid = input.usize() - 1;
            // let mut r = 0;
            // while mid - r > 0 && mid + r + 1 < n {
            //     r += 1;
            //     let my = am[mid + r] - am[mid - r];
            //     let expected = k * Mod::new(r) + b;
            //     if my != expected {
            //         r -= 1;
            //         break;
            //     }
            // }
            let res = {
                if mid == 0 || mid == n - 1 {
                    0
                } else {
                    let expected = k + b;
                    let real = deltas[mid - 1] + deltas[mid];
                    if expected != real {
                        0
                    } else {
                        // dbg!(n, mid);
                        binary_search_last_true(1..(mid + 1).min(n - mid), |check| {
                            let len = check - 1;
                            // dbg!(n, mid, len);
                            let to1 = mid - 1;
                            let from1 = to1 - len;
                            let from2 = mid + 1;
                            let to2 = from2 + len;
                            // dbg!("Checking", mid, len, from1, to1, from2, to2);
                            assert!(to2 < n);
                            let hash = deltas_seg_tree.get(from1..from1 + len).hash
                                + deltas_seg_tree.get(from2..from2 + len).hash_rev;
                            let expected_hash = expected_hashes[len];
                            hash == expected_hash
                            // if check == 2 {
                            //     dbg!(deltas, mid, k);
                            // }
                            // for i in 0..len {
                            //     let ss = deltas[from1 + i] + deltas[to2 - 1 - i];
                            //     dbg!(i, deltas[from1 + i], deltas[to2 - 1 - i], ss, k);
                            //     if ss != k {
                            //         return false;
                            //     }
                            // }
                            // true
                        })
                        .unwrap()
                        // let mut r = 1;
                        // loop {
                        //     if mid - r == 0 || mid + r == n - 1 {
                        //         break;
                        //     }
                        //     let sum_deltas = deltas[mid - r - 1] + deltas[mid + r];
                        //     if sum_deltas != k {
                        //         break;
                        //     } else {
                        //         r += 1;
                        //     }
                        // }
                        // r
                    }
                }
            };
            // assert_eq!(res, r);
            out.println(res);
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
    const PROBLEM_NAME: &str = "a";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
