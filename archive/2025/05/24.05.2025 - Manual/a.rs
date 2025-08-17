//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};

#[derive(Clone, Copy, Debug)]
struct Query {
    left: usize,
    right: usize,
    id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Value {
    val: usize,
    pos: usize,
    res: i32,
}

type ST = SegTreeMax<i32>;

const BLOCK: usize = 50;

fn solve_case(a: &[usize], w: &[i32], queries: &[Query], slow: bool) -> Vec<i32> {
    let n = a.len();

    let q = queries.len();
    let mut results = vec![0; q];
    let mut q_left = vec![vec![]; n + 1];
    let mut q_right = vec![vec![]; n + 1];
    for query in queries.iter() {
        q_left[query.left].push(query);
        q_right[query.right].push(query);
    }
    let mut from_left = vec![0; n];
    {
        let mut st = ST::new(n, |i| MaxValNode { max_val: 0, pos: i });
        for i in 0..n {
            let mx = st.get(0..a[i] + 1).max_val;
            from_left[i] = mx + w[i];
            st.update_point(
                a[i],
                MaxValNode {
                    max_val: from_left[i],
                    pos: a[i],
                },
            );
        }
    }
    let mut from_right = vec![0; n];
    {
        let mut st = ST::new(n, |i| MaxValNode { max_val: 0, pos: i });
        for i in (0..n).rev() {
            let mx = st.get(a[i]..n).max_val;
            from_right[i] = mx + w[i];
            st.update_point(
                a[i],
                MaxValNode {
                    max_val: from_right[i],
                    pos: a[i],
                },
            );
        }
    }
    {
        let mut st = ST::new(n, |i| MaxValNode { max_val: 0, pos: i });
        for i in 0..n {
            for query in q_left[i].iter() {
                let mut mx = st.get(0..n).max_val;
                let right_end = ((query.right / BLOCK + 1) * BLOCK).min(n);
                for r in query.right..right_end {
                    let mx_here = st.get(0..a[r] + 1).max_val + from_right[r];
                    mx = mx.max(mx_here);
                }
                results[query.id] = results[query.id].max(mx);
            }
            st.update_point(
                a[i],
                MaxValNode {
                    max_val: from_left[i],
                    pos: a[i],
                },
            );
        }
    }
    {
        let mut st = ST::new(n, |i| MaxValNode { max_val: 0, pos: i });
        for i in (0..n).rev() {
            st.update_point(
                a[i],
                MaxValNode {
                    max_val: from_right[i],
                    pos: a[i],
                },
            );
            for query in q_right[i].iter() {
                let mut mx = st.get(0..n).max_val;
                let left_end = (query.left / BLOCK) * BLOCK;
                for l in left_end..query.left {
                    let mx_here = st.get(a[l]..n).max_val + from_left[l];
                    mx = mx.max(mx_here);
                }
                results[query.id] = results[query.id].max(mx);
            }
        }
    }
    if slow {
        for i in 0..queries.len() {
            let mut res = 0;
            for l in 0..queries[i].left {
                res = res.max(from_left[l]);
                for r in queries[i].right..n {
                    if a[l] <= a[r] {
                        res = res.max(from_left[l] + from_right[r]);
                    }
                }
            }
            for r in queries[i].right..n {
                res = res.max(from_right[r]);
            }
            results[i] = res;
        }
    } else {
        let cnt_blocks = (n + BLOCK - 1) / BLOCK;
        let mut blocks_left = vec![vec![]; cnt_blocks];
        for i in 0..n {
            blocks_left[i / BLOCK].push(Value {
                val: a[i],
                pos: i,
                res: from_left[i],
            });
        }
        let mut blocks_right = vec![vec![]; cnt_blocks];
        for i in 0..n {
            blocks_right[i / BLOCK].push(Value {
                val: a[i],
                pos: i,
                res: from_right[i],
            });
        }
        for b in blocks_left.iter_mut() {
            b.sort();
        }
        for b in blocks_right.iter_mut() {
            b.sort();
        }
        let mut dp = Array2D::new(0, cnt_blocks, cnt_blocks);
        for i in 0..cnt_blocks {
            for j in (i + 1..cnt_blocks).rev() {
                dp[i][j] = merge_blocks(&blocks_left[i], &blocks_right[j]);
                if i > 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                }
                if j + 1 < cnt_blocks {
                    dp[i][j] = dp[i][j].max(dp[i][j + 1]);
                }
            }
        }
        // dbg!(dp[0][2]);
        for query in queries.iter() {
            let left_block = query.left / BLOCK;
            let right_block = query.right / BLOCK + 1;
            if left_block > 0 && right_block < cnt_blocks {
                let mx = dp[left_block - 1][right_block];
                results[query.id] = results[query.id].max(mx);
            }
        }
    }
    results
}

fn merge_blocks(left: &[Value], right: &[Value]) -> i32 {
    let mut res = 0;
    let mut max_left = 0;
    let mut it = 0;
    for val in right.iter() {
        while it < left.len() && left[it].val <= val.val {
            max_left = max_left.max(left[it].res);
            it += 1;
        }
        res = res.max(max_left + val.res);
    }
    while it < left.len() {
        max_left = max_left.max(left[it].res);
        it += 1;
    }
    res
}

fn stress() {
    for it in 203.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_N: usize = 50;
        const MAX_W: i32 = 10;
        const MAX_A: i32 = 10;
        let n = rnd.gen(1..MAX_N);
        let a = rnd.gen_vec(n, 0..n);
        let w = rnd.gen_vec(n, 1..MAX_W);
        let mut queries = vec![];
        let q = rnd.gen(1..2);
        for _ in 0..q {
            let l = rnd.gen(0..n);
            let r = rnd.gen(l + 1..n + 1);
            queries.push(Query {
                left: l,
                right: r,
                id: queries.len(),
            });
        }
        let slow_ans = solve_case(&a, &w, &queries, true);
        let fast_ans = solve_case(&a, &w, &queries, false);
        if slow_ans != fast_ans {
            println!("Wrong answer");
            println!("a: {:?}", a);
            println!("w: {:?}", w);
            for q in queries.iter() {
                println!("query: {:?}", q);
            }
            println!("slow: {:?}", slow_ans);
            println!("fast: {:?}", fast_ans);
            return;
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut a = vec![];
    let mut w = vec![];
    for _ in 0..n {
        a.push(input.usize() - 1);
        w.push(input.i32());
    }
    let mut queries = vec![];
    for id in 0..q {
        let l = input.usize() - 1;
        let r = input.usize();
        queries.push(Query {
            left: l,
            right: r,
            id,
        });
    }
    let results = solve_case(&a, &w, &queries, false);
    for x in results {
        out.println(x);
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
