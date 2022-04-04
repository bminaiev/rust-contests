//{"name":"I. Diversity Street","group":"Yandex - Stage 12: Grand Prix of Grushevka","url":"https://official.contest.yandex.com/opencupXXII/contest/35268/problems/I/","interactive":false,"timeLimit":3000,"tests":[{"input":"2 2\n2 1 1\n2 1 2\n","output":"YES\n2 1\n"},{"input":"3 2\n2 1 2\n2 2 3\n","output":"YES\n1 3 2\n"},{"input":"4 2\n4 1 2\n3 2 4\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IDiversityStreet"}}}

use std::cmp::min;
use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Restriction {
    id: usize,
    from: usize,
    to: usize,
    at_least: usize,
}

#[derive(Default, Clone, Copy)]
struct Node {
    sum: i32,
    min_suf: i32,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self {
            sum: l.sum + r.sum,
            min_suf: min(r.min_suf, l.min_suf + r.sum),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.sum += *update;
        node.min_suf = node.sum;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += add;
    }

    type Update = i32;

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut restrictions = gen_vec(m, |id| Restriction {
        id,
        at_least: input.usize() - 1,
        from: input.usize() - 1,
        to: input.usize(),
    });
    restrictions.sort_by_key(|r| r.at_least);
    restrictions.reverse();
    let mut by_id = vec![vec![]; n];
    let mut has_space: BTreeSet<usize> = (0..n).collect();
    for r in restrictions.iter() {
        let mut fr = r.from;
        while fr < r.to {
            fr = *has_space.range(fr..r.to).next().unwrap_or(&r.to);
            if fr == r.to {
                break;
            }
            by_id[fr].push(r.id);
            if by_id[fr].len() == 2 {
                has_space.remove(&fr);
            }
            fr += 1;
        }
    }
    restrictions.sort_by_key(|r| r.id);
    let mut seg_tree = LazySegTree::new_f(n, &|_pos| Node { sum: 1, min_suf: 1 });
    let mut cur_min_val = vec![0; n];
    for i in 0..n {
        cur_min_val[i] = if by_id[i].is_empty() {
            0
        } else {
            restrictions[by_id[i][0]].at_least
        };
        seg_tree.update(cur_min_val[i]..cur_min_val[i] + 1, -1);
    }
    let mut what_restricted = vec![vec![]; m];
    for i in 0..n {
        if by_id[i].is_empty() {
            continue;
        }
        let id = by_id[i][0];
        what_restricted[id].push(i);
    }
    for j in 0..m {
        for &i in what_restricted[j].iter() {
            let new_val = if by_id[i].len() == 1 {
                0
            } else {
                restrictions[by_id[i][1]].at_least
            };
            seg_tree.update(cur_min_val[i]..cur_min_val[i] + 1, 1);
            seg_tree.update(new_val..new_val + 1, -1);
        }
        if seg_tree.get(0..n).min_suf >= 0 {
            out_line!("YES");
            for &i in what_restricted[j].iter() {
                let new_val = if by_id[i].len() == 1 {
                    0
                } else {
                    restrictions[by_id[i][1]].at_least
                };
                cur_min_val[i] = new_val;
            }
            let mut order: Vec<_> = (0..n).collect();
            order.sort_by_key(|id| cur_min_val[*id]);
            let mut res = vec![0; n];
            for i in 0..n {
                let id = order[i];
                assert!(cur_min_val[id] <= i);
                res[id] = i + 1;
            }
            out_line!(res);
            return;
        }
        for &i in what_restricted[j].iter() {
            let new_val = if by_id[i].len() == 1 {
                0
            } else {
                restrictions[by_id[i][1]].at_least
            };
            seg_tree.update(cur_min_val[i]..cur_min_val[i] + 1, -1);
            seg_tree.update(new_val..new_val + 1, 1);
        }
    }
    out_line!("NO");
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
