//{"name":"E. Tokitsukaze и красивые подотрезки","group":"Codeforces - Codeforces Round #789 (Div. 1)","url":"https://codeforces.com/contest/1677/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"8 3\n1 3 5 2 4 7 6 8\n1 3\n1 1\n1 8\n","output":"2\n0\n10\n"},{"input":"10 10\n6 1 3 2 5 8 4 10 7 9\n1 8\n1 10\n1 2\n1 4\n2 4\n5 8\n4 10\n4 7\n8 10\n5 9\n","output":"17\n25\n1\n5\n2\n0\n4\n1\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETokitsukazeIKrasiviePodotrezki"}}}

use std::cmp::{max, min};
use std::ops::Range;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::permutation::Permutation;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree_add_sum::{Node, SegTreeAddSum};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type SegTree = SegTreeAddSum<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let p = Permutation::from_vec(input.vec::<usize>(n).sub_from_all(1));
    let mut query_by_r = vec![vec![]; n];
    for id in 0..q {
        let l = input.usize() - 1;
        let r = input.usize();
        query_by_r[r - 1].push(Query { id, range: l..r });
    }
    let mut multipliers = vec![vec![]; n + 1];
    for x in 1..=n {
        for y in x + 1..=n {
            if x * y > n {
                break;
            }
            multipliers[x * y - 1].push((x - 1, y - 1));
        }
    }

    // res = st_a * R + st_b
    let mut st_a = SegTree::new_f(n, &|_| Node::new(0));
    let mut st_b = SegTree::new_f(n, &|_| Node::new(0));

    let change_st =
        |r: Range<usize>, delta: i64, cur_r: i64, st_a: &mut SegTree, st_b: &mut SegTree| {
            st_a.update(r.clone(), delta);
            st_b.update(r.clone(), (-cur_r + 1) * delta);
        };

    let mut stack: Vec<StackElem> = vec![];
    let mut res = vec![0; q];

    let mut bad_from_updates = vec![vec![]; n];

    for r in 0..n {
        let cur_value = p[r];
        while !stack.is_empty() && stack.last_exn().max_elem < cur_value {
            let last = stack.last_exn().clone();
            change_st(
                last.range.start..last.bad_from,
                -1,
                r as i64,
                &mut st_a,
                &mut st_b,
            );
            stack.pop();
        }
        let from = if stack.is_empty() {
            0
        } else {
            stack.last_exn().range.end
        };
        let mut bad_from = from;
        for &(x, y) in multipliers[cur_value].iter() {
            let p1 = p.get_pos_of_element(x);
            let p2 = p.get_pos_of_element(y);
            let min_pos = min(p1, p2);
            let max_pos = max(p1, p2);
            if max_pos <= r {
                bad_from.update_max(min_pos + 1);
            } else {
                bad_from_updates[max_pos].push(BadFromUpdate {
                    bad_from: min(min_pos, r) + 1,
                    stack_pos: stack.len(),
                    max_elem: cur_value,
                });
            }
        }
        change_st(from..bad_from, 1, r as i64, &mut st_a, &mut st_b);
        stack.push(StackElem {
            range: from..r + 1,
            max_elem: cur_value,
            bad_from,
        });
        for update in bad_from_updates[r].iter() {
            if update.stack_pos < stack.len() && stack[update.stack_pos].max_elem == update.max_elem
            {
                if stack[update.stack_pos].bad_from < update.bad_from {
                    change_st(
                        stack[update.stack_pos].bad_from..update.bad_from,
                        1,
                        r as i64,
                        &mut st_a,
                        &mut st_b,
                    );
                    stack[update.stack_pos].bad_from = update.bad_from;
                }
            }
        }
        for query in query_by_r[r].iter() {
            let cur_res =
                st_a.get(query.range.clone()).sum * (r as i64) + st_b.get(query.range.clone()).sum;
            res[query.id] = cur_res;
        }
    }
    out_line!(res);
}

#[derive(Clone, Debug)]
struct StackElem {
    range: Range<usize>,
    max_elem: usize,
    bad_from: usize,
}

#[derive(Clone, Debug)]
struct BadFromUpdate {
    stack_pos: usize,
    max_elem: usize,
    bad_from: usize,
}

#[derive(Clone, Debug)]
struct Query {
    range: Range<usize>,
    id: usize,
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("3");
}
//END MAIN
