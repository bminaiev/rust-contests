//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::bottom_up_seg_tree::BottomUpSegTree;
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Query {
    r: usize,
    l: usize,
    id: usize,
}

// type ST = SegTreeMax<Query>;
type ST = BottomUpSegTree<MaxValNode<i32>>;

fn stress() {
    for tc in 1.. {
        dbg!(tc);
        let start = Instant::now();
        let mut rnd = Random::new(tc);
        let n = 150000; //rnd.gen(1..10);
        const MAX: usize = 1000000;
        let mut a = vec![720720; n]; //rnd.gen_vec(n, 1..MAX);
        let mut queries = vec![];
        let queries_n = 100000; //rnd.gen(1..10);
        for id in 0..queries_n {
            let l = rnd.gen(0..n);
            let r = rnd.gen(l..n.min(l + 10));
            queries.push(Query { l, r, id: id + 1 });
        }
        let res1 = solve_case(&a, &queries);
        dbg!(start.elapsed());
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<usize>(n);

    let mut queries = Vec::with_capacity(q);
    for i in 0..q {
        let l = input.usize();
        let r = input.usize();
        queries.push(Query {
            l: l - 1,
            r: r - 1,
            id: i + 1,
        });
    }

    let results = solve_case(&a, &queries);

    for x in results {
        out.println(x);
    }
}

fn solve_case(a: &[usize], queries: &[Query]) -> Vec<usize> {
    let n = a.len();
    let max_a = *a.iter().max().unwrap();
    let mut positions = vec![vec![]; max_a + 1];
    for i in 0..n {
        positions[a[i]].push(i);
    }
    let mut queries_by_l = vec![vec![]; n];
    for q in queries.iter() {
        queries_by_l[q.l].push(*q);
    }
    for i in 0..n {
        queries_by_l[i].sort_by_key(|q| q.r);
    }
    let empty_query = Query { l: 0, r: 0, id: 0 };
    let mut seg_tree = ST::new(n.next_power_of_two(), |pos| {
        let query = if pos >= n || queries_by_l[pos].is_empty() {
            empty_query
        } else {
            *queries_by_l[pos].last().unwrap()
        };
        MaxValNode {
            max_val: query.r as i32,
            pos: pos as i32,
        }
    });
    let mut results = vec![0; queries.len()];
    let mut seen_r = vec![n; n];
    for gcd in (1..=max_a).rev() {
        let mut cur_pos = vec![];
        for mult in 1.. {
            let value = mult * gcd;
            if value > max_a {
                break;
            }
            cur_pos.extend_from_slice(&positions[value]);
        }
        cur_pos.sort_unstable();
        for i in 1..cur_pos.len() {
            let mid = cur_pos[i];
            let prev = cur_pos[i - 1];
            let right_bound = mid + (mid - prev);
            let right_pos =
                binary_search_first_true(i + 1..cur_pos.len(), |i| cur_pos[i] >= right_bound);
            if right_pos == cur_pos.len() {
                continue;
            }
            let right_bound = cur_pos[right_pos];
            if seen_r[prev] <= right_bound {
                continue;
            }
            seen_r[prev] = right_bound;
            loop {
                let node = seg_tree.get(0..prev + 1);
                if node.max_val >= right_bound as i32 {
                    let l_pos = node.pos as usize;
                    let query = *queries_by_l[l_pos].last().unwrap();
                    results[query.id - 1] = gcd;
                    queries_by_l[query.l].pop();
                    {
                        let next_query = if queries_by_l[query.l].is_empty() {
                            empty_query
                        } else {
                            *queries_by_l[query.l].last().unwrap()
                        };
                        seg_tree.update_point(
                            query.l,
                            MaxValNode {
                                max_val: next_query.r as i32,
                                pos: query.l as i32,
                            },
                        );
                    }
                } else {
                    break;
                }
            }
        }
    }
    results
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
