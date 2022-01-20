//{"name":"A. Analytics","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/?nc=PuuTHURJ","interactive":false,"timeLimit":2000,"tests":[{"input":"6 7 8\n1 2\n3 5\n2 4\n2 4\n2 6\n1 5\n2 1\n1 1 7 1\n1 1 7 2\n1 1 7 3\n1 1 7 4\n1 1 7 5\n1 1 7 6\n2 4 5 2\n1 1 1 1\n","output":"6\n3\n5\n4\n2\n1\n4\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAnalytics"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::vec_binary_search::VecBinarySearch;
use algo_lib::{dbg, out, out_line};

struct Query {
    fr_time: usize,
    to_time: usize,
    where_is_x: usize,
    res_pos: usize,
}

fn solve_one_side(n: usize, changes: &[(usize, usize)], queries: &[Query]) -> Vec<(usize, usize)> {
    let mut by_pos = vec![vec![]; n];
    for (id, &(fr, _to)) in changes.iter().enumerate() {
        by_pos[fr].push(id);
    }

    let mut change_next = vec![None; changes.len()];
    let mut last_change = vec![None; n];
    for id in (0..changes.len()).step_by(2).rev() {
        let (fr, to) = changes[id];
        change_next[id] = last_change[to];
        change_next[id + 1] = last_change[fr];
        last_change[fr] = Some(id);
        last_change[to] = Some(id + 1);
    }
    const M: usize = 20;
    let mut go = Array2D::new(0, M, changes.len());
    for i in 0..changes.len() {
        if let Some(to) = change_next[i] {
            go[0][i] = to;
        } else {
            go[0][i] = i;
        }
    }
    for lvl in 1..M {
        for v in 0..changes.len() {
            go[lvl][v] = go[lvl - 1][go[lvl - 1][v]];
        }
    }

    queries
        .iter()
        .map(|query| {
            let fr_time = query.fr_time;
            let to_time = query.to_time;

            // where will be x?
            let start_pos = query.where_is_x;
            let mut id = by_pos[start_pos]
                .higher_or_equal(&fr_time)
                .unwrap_or(to_time);
            let ans = if id < to_time {
                for lvl in (0..M).rev() {
                    let use_it = go[lvl][id];
                    if use_it < to_time {
                        id = use_it;
                    }
                }
                changes[id].1
            } else {
                start_pos
            };
            (query.res_pos, ans + 1)
        })
        .collect()
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let q = input.usize();
    let mut changes = vec![];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        changes.push((fr, to));
        changes.push((to, fr));
    }

    let mut queries_fwd = vec![];
    let mut queries_back = vec![];

    let total = changes.len();

    for res_pos in 0..q {
        let q_type = input.usize();
        let fr = (input.usize() - 1) * 2;
        let to = input.usize() * 2;
        let where_is_x = input.usize() - 1;
        if q_type == 1 {
            // who is on place x?
            queries_back.push(Query {
                fr_time: total - to,
                to_time: total - fr,
                where_is_x,
                res_pos,
            });
        } else {
            assert_eq!(q_type, 2);

            queries_fwd.push(Query {
                fr_time: fr,
                to_time: to,
                where_is_x,
                res_pos,
            });
        }
    }

    let mut res = vec![0; q];
    for &(q_id, ans) in solve_one_side(n, &changes, &queries_fwd).iter() {
        res[q_id] = ans;
    }
    for &(q_id, ans) in solve_one_side(n, &changes.reversed(), &queries_back).iter() {
        res[q_id] = ans;
    }

    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
