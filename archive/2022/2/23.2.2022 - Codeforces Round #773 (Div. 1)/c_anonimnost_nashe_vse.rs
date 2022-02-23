//{"name":"C. Анонимность наше все","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"6 9\n0 4 5 0\n1 5\n1 6\n0 4 6 1\n1 6\n0 2 5 1\n0 2 2 0\n1 3\n1 2\n","output":"NO\nN/A\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAnonimnostNasheVse"}}}

use std::cmp::max;
use std::collections::BTreeSet;

use algo_lib::collections::sparse_table_max::SparseTableMax;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Query {
    type_: usize,
    l: usize,
    r: usize,
    x: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut queries = vec![];
    for _ in 0..q {
        let type_ = input.usize();
        if type_ == 0 {
            let l = input.usize() - 1;
            let r = input.usize();
            let x = input.usize();
            queries.push(Query { type_, l, r, x });
        } else {
            let pos = input.usize() - 1;
            queries.push(Query {
                type_,
                l: pos,
                r: pos,
                x: 0,
            });
        }
    }
    let mut unknown: BTreeSet<usize> = (0..n).collect();
    let mut known_good = vec![q; n];
    let mut known_bad = vec![q; n];
    for it in 0..q {
        if queries[it].type_ == 0 && queries[it].x == 0 {
            let mut to_remove = vec![];
            for pos in unknown.range(queries[it].l..queries[it].r) {
                to_remove.push(*pos);
            }
            for r in to_remove.iter() {
                unknown.remove(r);
                known_good[*r] = it;
            }
        }
    }
    let table = SparseTableMax::new(&known_good);
    for it in 0..q {
        if queries[it].type_ == 0 && queries[it].x == 1 {
            let l = queries[it].l;
            let r = queries[it].r;
            let max_pos = table.find_max_pos(l..r);
            assert_eq!(known_good[max_pos], q);
            let mut max_pos2 = max_pos;
            if max_pos > l {
                max_pos2 = table.find_max_pos(l..max_pos);
            }
            if max_pos + 1 < r {
                let check = table.find_max_pos(max_pos + 1..r);
                if max_pos2 == max_pos || known_good[check] > known_good[max_pos2] {
                    max_pos2 = check;
                }
            }
            if max_pos2 == max_pos {
                known_bad[max_pos].update_min(it);
            } else {
                known_bad[max_pos].update_min(max(it, known_good[max_pos2]));
            }
        }
    }
    for it in 0..q {
        if queries[it].type_ == 1 {
            let pos = queries[it].l;
            if known_good[pos] < it {
                out_line!("NO");
            } else if known_bad[pos] < it {
                out_line!("YES");
            } else {
                out_line!("N/A");
            }
        }
    }
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
    // tester::run_single_test("1");
}
//END MAIN
