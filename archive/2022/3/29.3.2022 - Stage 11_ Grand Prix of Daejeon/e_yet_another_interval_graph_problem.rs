//{"name":"E. Yet Another Interval Graph Problem","group":"Yandex - Stage 11: Grand Prix of Daejeon","url":"https://official.contest.yandex.com/opencupXXII/contest/35265/problems/E/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n1 4 1\n3 6 2\n5 8 5\n7 10 2\n9 12 1\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EYetAnotherIntervalGraphProblem"}}}

use std::cmp::{max, min};
use std::collections::BTreeSet;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Elem {
    w: i64,
    start: i32,
    end: i32,
    id: usize,
}

fn solve_case(max_sz: usize, a: &[Elem]) -> i64 {
    let mut a: Vec<_> = a
        .iter()
        .map(|e| Elem {
            w: e.w,
            start: e.start * 2 - 1,
            end: e.end * 2 + 1,
            id: e.id,
        })
        .collect();
    let a_not_sorted = a.clone();
    let mut all_pts = vec![];
    for e in a.iter() {
        all_pts.push(e.start);
        all_pts.push(e.end);
    }
    all_pts.sort();
    all_pts.dedup();
    let mut dp = vec![i64::MAX; all_pts.len()];
    let mut split_here: Vec<_> = all_pts
        .iter()
        .map(|&coord| {
            let mut cost = 0;
            for e in a.iter() {
                if e.start < coord && e.end > coord {
                    cost += e.w;
                }
            }
            cost
        })
        .collect();
    dp[0] = 0;
    let n = a.len();
    let mut inside_split_here = vec![true; n];
    assert_eq!(split_here[0], 0);
    a.sort_by_key(|e| e.end);
    for i in 0..all_pts.len() {
        let cur = dp[i];
        if cur == i64::MAX {
            continue;
        }
        for j in 0..n {
            if inside_split_here[j] && a_not_sorted[j].start < all_pts[i] {
                inside_split_here[j] = false;
                for k in 0..all_pts.len() {
                    if a_not_sorted[j].start < all_pts[k] && a_not_sorted[j].end > all_pts[k] {
                        split_here[k] -= a_not_sorted[j].w;
                    }
                }
            }
        }
        let mut iter = 0;
        let mut cur_cost = 0;
        let mut set = BTreeSet::new();
        for j in i + 1..all_pts.len() {
            let next = all_pts[j];
            while iter != a.len() && a[iter].end <= next {
                if a[iter].start >= all_pts[i] {
                    set.insert(a[iter]);
                    if set.len() > max_sz {
                        let first = set.iter().next().unwrap().clone();
                        cur_cost += first.w;
                        set.remove(&first);
                    }
                }
                iter += 1;
            }
            dp[j].update_min(cur + split_here[j] + cur_cost);
        }
    }
    dp[dp.len() - 1]
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let max_sz = input.usize();
    let a = gen_vec(n, |id| Elem {
        start: input.i32(),
        end: input.i32(),
        w: input.read(),
        id,
    });
    out_line!(solve_case(max_sz, &a));
}

fn solve_case_slow(max_sz: usize, a: &[Elem]) -> i64 {
    let mut res = i64::MAX;
    let n = a.len();
    for mask_alive in 0..(1 << n) {
        let mut cost = 0;
        for i in 0..n {
            if ((1 << i) & mask_alive) == 0 {
                cost += a[i].w;
            }
        }
        let mut dsu = Dsu::new(n);
        for i in 0..n {
            for j in i + 1..n {
                if ((1 << i) & mask_alive) != 0 {
                    if ((1 << j) & mask_alive) != 0 {
                        if max(a[i].start, a[j].start) <= min(a[i].end, a[j].end) {
                            dsu.unite(i, j);
                        }
                    }
                }
            }
        }
        let mut sizes = vec![0; n];
        for i in 0..n {
            sizes[dsu.get(i)] += 1;
        }
        let mut ok = true;
        for i in 0..n {
            if sizes[i] > max_sz {
                ok = false;
                break;
            }
        }
        if ok {
            res.update_min(cost);
        }
    }
    res
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_in_range(1..15);
        const MAX_C: i32 = 10;
        const MAX_W: i64 = 100;
        let a = gen_vec(n, |id| {
            let start = rnd.gen_in_range(1..MAX_C);
            let end = rnd.gen_in_range(1..MAX_C);
            Elem {
                start: min(start, end),
                end: max(start, end),
                w: rnd.gen_in_range(1..MAX_W),
                id,
            }
        });
        let max_sz = rnd.gen_in_range(1..n + 1);
        let fast = solve_case(max_sz, &a);
        let slow = solve_case_slow(max_sz, &a);
        if fast != slow {
            dbg!(max_sz);
            for e in a.iter() {
                dbg!(e);
            }
            dbg!(slow, fast);
            assert_eq!(fast, slow);
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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
