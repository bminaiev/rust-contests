//{"name":"D. Разбиение на возрастающую и спадающую","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3 1\n","output":"6\n"},{"input":"6\n4 5 2 6 1 3\n","output":"19\n"},{"input":"10\n7 10 1 8 3 9 2 4 6 5\n","output":"39\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRazbienieNaVozrastayushchuyuISpadayushchuyu"}}}

use std::cmp::min;
use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Res {
    len: usize,
    next_number: usize,
}

fn calc_max_right(pp: &[usize]) -> Vec<Res> {
    let n = pp.len();
    let mut pos_of = vec![0; n];
    for pos in 0..n {
        pos_of[pp[pos]] = pos;
    }
    let mut res = vec![
        Res {
            len: 0,
            next_number: 0
        };
        n
    ];
    let mut seen_pos = BTreeSet::new();
    let mut bad_pos = BTreeSet::new();
    for v in 0..n {
        let p = pos_of[v];
        if let Some(prev) = seen_pos.range(0..p).next_back() {
            bad_pos.insert(*prev);
        }
        let next_bad = *bad_pos.range(p..).next().unwrap_or(&n);
        let len = *seen_pos.range(next_bad + 1..).next().unwrap_or(&n) - p;
        res[p] = Res {
            len,
            next_number: pp[*seen_pos.range(p..).next().unwrap_or(&p)],
        };
        seen_pos.insert(p);
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p = input.vec::<usize>(n).sub_from_all(1);
    let right = calc_max_right(&p);
    let left = {
        let mut p_rev = p.clone();
        p_rev.reverse();
        let mut res = calc_max_right(&p_rev);
        res.reverse();
        res
    };
    let sub_p: Vec<_> = p.iter().map(|x| n - 1 - x).collect();
    let mut up_right = calc_max_right(&sub_p);
    let mut up_left = {
        let mut p_rev = sub_p.clone();
        p_rev.reverse();
        let mut res = calc_max_right(&p_rev);
        res.reverse();
        res
    };
    for r in up_right.iter_mut() {
        r.next_number = n - 1 - r.next_number;
    }
    for r in up_left.iter_mut() {
        r.next_number = n - 1 - r.next_number;
    }

    let mut max_right = vec![0; n];
    for p in 0..n {
        let to = p + min(right[p].len, up_right[p].len);
        let from = p + 1 - min(left[p].len, up_left[p].len);
        max_right[from].update_max(to);
    }
    for pos in 0..n - 1 {
        if p[pos] > p[pos + 1] {
            if left[pos].next_number != p[pos] && up_right[pos + 1].next_number != p[pos + 1] {
                if left[pos].next_number < up_right[pos + 1].next_number {
                    let to = pos + 1 + min(right[pos + 1].len, up_right[pos + 1].len);
                    let from = pos + 1 - min(left[pos].len, up_left[pos].len);
                    max_right[from].update_max(to);
                }
            }
        } else {
            if up_left[pos].next_number != p[pos] && right[pos + 1].next_number != p[pos + 1] {
                if up_left[pos].next_number > right[pos + 1].next_number {
                    let to = pos + 1 + min(right[pos + 1].len, up_right[pos + 1].len);
                    let from = pos + 1 - min(left[pos].len, up_left[pos].len);
                    max_right[from].update_max(to);
                }
            }
        }
    }

    let mut cur_max = 0;
    let mut res = 0;
    for left in 0..n {
        cur_max.update_max(max_right[left]);
        res += (cur_max - left) as i64;
    }
    out_line!(res);
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
}
//END MAIN
