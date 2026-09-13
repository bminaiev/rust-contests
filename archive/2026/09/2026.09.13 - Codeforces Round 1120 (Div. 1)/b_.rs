use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::bottom_up_seg_tree::BottomUpSegTree;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

#[derive(Clone, Copy, Default)]
struct Node {
    sum: i64,
    smallest: i64,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self {
            sum: l.sum + r.sum,
            smallest: l.smallest.min(l.sum + r.smallest),
        }
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

struct NextBigger {
    a: Vec<i64>,
    st: BottomUpSegTree<Node>,
}

impl NextBigger {
    fn new(a: Vec<i64>) -> Self {
        let st = BottomUpSegTree::new(a.len(), |i| Node {
            sum: a[i],
            smallest: -a[i],
        });
        Self { a, st }
    }

    fn next(&mut self, pos: usize, hint: Option<usize>) -> Option<usize> {
        let mut zz = self.a.len();
        if let Some(hint) = hint {
            zz = zz.min(hint + 1);
        }
        let next = binary_search_first_true(pos + 1..zz, |next| {
            let smallest = self.st.get(pos + 1..next + 1).smallest;
            let sum = self.a[pos] + smallest;
            sum < 0
        });
        if next == self.a.len() {
            None
        } else {
            Some(next)
        }
        // let mut sum = self.a[pos];
        // for i in pos + 1..self.a.len() {
        //     if self.a[i] > sum {
        //         return Some(i);
        //     }
        //     sum += self.a[i];
        // }
        // None
    }

    fn reset(&mut self, pos: usize) {
        self.a[pos] = 0;
        self.st.update_point(
            pos,
            Node {
                sum: 0,
                smallest: 0,
            },
        );
    }
}

fn solve_case(a: &[i64], p: &[usize]) -> Vec<usize> {
    let mut nb = NextBigger::new(a.to_vec());
    let mut maxes_bs = BTreeSet::new();
    maxes_bs.insert(0);
    let mut alive = vec![true; a.len()];
    {
        let mut tmp = 0;
        while let Some(next) = nb.next(tmp, None) {
            maxes_bs.insert(next);
            tmp = next;
        }
    }
    let mut res = vec![];
    res.push(maxes_bs.len());
    let mut start = 0;
    for &pos in p[..a.len() - 1].iter() {
        nb.reset(pos);
        alive[pos] = false;
        while !alive[start] {
            start += 1;
        }
        if maxes_bs.contains(&pos) {
            maxes_bs.remove(&pos);
        }
        let mut prev = start;
        if let Some(x) = maxes_bs.range(..pos).next_back() {
            prev = *x;
        }
        maxes_bs.insert(prev);
        loop {
            let hint = maxes_bs.range(prev + 1..).next().copied();
            if let Some(next) = nb.next(prev, hint) {
                if maxes_bs.contains(&next) {
                    break;
                }
                maxes_bs.insert(next);
                prev = next;
            } else {
                break;
            }
        }
        res.push(maxes_bs.len());
    }
    res.sub_from_all(1)
}

// 16:44
fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n);
        let p = input.vec::<usize>(n).sub_from_all(1);
        let res = solve_case(&a, &p);
        out.println(res);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..200_000);
        let a = rnd.gen_vec(n, 1..100);
        let p = rnd.gen_permutation(n);
        let res = solve_case(&a, &p);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
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
