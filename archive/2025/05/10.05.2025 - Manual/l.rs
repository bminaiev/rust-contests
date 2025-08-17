//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::num_traits::Number;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

fn single(a: &[usize]) -> u64 {
    let mut left = 0;
    let mut right = a.len();
    while left + 1 < right {
        if a[left + 1..right].contains(&a[left]) {
            break;
        } else {
            left += 1;
        }
    }
    while left + 1 < right {
        if a[left..right - 1].contains(&a[right - 1]) {
            break;
        } else {
            right -= 1;
        }
    }
    (right - left) as u64
}

fn solve_very_slow(a: &[usize]) -> u64 {
    let n = a.len();
    let mut res = 0;
    for l in 0..n {
        for r in l + 1..=n {
            res += single(&a[l..r]);
        }
    }
    res
}

fn solve_slow(a: &[usize]) -> u64 {
    let n = a.len();
    let mut res = 0;
    let mut seen = vec![n; n];
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for l in (0..a.len()).rev() {
        let x = a[l];
        let prev_pos = seen[x];
        for p in l + 1..prev_pos {
            if right[p] == l + 1 {
                left[p] = l;
                right[p] = l;
            }
        }
        for p in prev_pos..n {
            left[p] = l;
            right[p] = right[p].max(prev_pos);
        }
        seen[x] = l;
        left[l] = l;
        right[l] = l;
        for r in l..n {
            res += (right[r] - left[r] + 1) as u64;
        }
    }
    res
}

#[derive(Clone, Default, Copy, Debug)]
pub struct Node {
    pub sum: u64,
    pub len: i32,
    pub min: u64,
}

impl SegTreeNode for Node {
    #[allow(unused)]
    fn join_nodes(l: &Self, r: &Self, context: &()) -> Self {
        Self {
            len: l.len + r.len,
            sum: l.sum + r.sum,
            min: l.min.min(r.min),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.sum = *update * (node.len as u64);
        node.min = *update;
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = u64;
    type Context = ();
}

type ST = SegTree<Node>;

fn values(s: &mut ST) -> Vec<u64> {
    (0..s.len()).map(|i| s.get(i..i + 1).sum).collect()
}

fn solve_faster(a: &[usize]) -> u64 {
    let n = a.len();
    let mut res = 0;
    let mut seen = vec![n; n];
    let mut left = ST::new(n, |p| Node {
        sum: p as u64,
        min: p as u64,
        len: 1,
    });
    let mut right = ST::new(n, |p| Node {
        sum: p as u64,
        min: p as u64,
        len: 1,
    });

    for l in (0..a.len()).rev() {
        let x = a[l];
        let prev_pos = seen[x];
        {
            // let till_pos = binary_search_first_true(l + 1..prev_pos, |p| {
            //     right.get(p..p + 1).sum > (l + 1) as u64
            // });
            // dbg!(l, values(&mut right), l..prev_pos);
            let till_pos = right
                .find_last_true(l..prev_pos, |node| node.min <= (l + 1) as u64)
                .unwrap()
                + 1;
            left.update(l..till_pos, l as u64);
            right.update(l..till_pos, l as u64);
        }
        left.update(prev_pos..n, l as u64);
        {
            // let till_pos = binary_search_first_true(prev_pos..n, |p| {
            //     right.get(p..p + 1).sum > prev_pos as u64
            // });
            let till_pos = right.find_last_true(prev_pos..n, |node| node.min <= prev_pos as u64);
            if let Some(till_pos) = till_pos {
                right.update(prev_pos..till_pos + 1, prev_pos as u64);
            }
        }
        seen[x] = l;
        res += right.get(l..n).sum;
        res -= left.get(l..n).sum;
        res += (n - l) as u64;
        // {
        //     dbg!(l);
        //     dbg!(left.values());
        //     dbg!(right.values());
        // }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n).sub_from_all(1);
        let res = solve_faster(&a);
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

fn stress() {
    for it in 23.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_N: usize = 50;
        let n = rnd.gen(1..MAX_N);
        let a = rnd.gen_vec(n, 0..n);
        // let res1 = solve_very_slow(&a);
        let res2 = solve_slow(&a);
        let res3 = solve_faster(&a);
        // assert_eq!(res1, res2);
        if res2 != res3 {
            dbg!(a);
            dbg!(res2);
            dbg!(res3);
            break;
        }
        assert_eq!(res2, res3);
        dbg!(res2);
    }
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "l";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
