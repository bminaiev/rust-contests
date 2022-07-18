//{"name":"D. Triple Sword Strike","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/D/","interactive":false,"timeLimit":4000,"tests":[{"input":"10\n1 1 8\n1 4 1\n1 5 9\n2 3 2\n2 4 1\n3 1 9\n3 2 9\n3 4 4\n4 3 3\n5 4 7\n","output":"48\n"},{"input":"8\n1 0 1\n1 1000000 1\n2 1 1\n2 999999 1\n3 2 1\n3 999998 1\n4 3 1\n4 999997 1\n","output":"6\n"},{"input":"1\n1 1 3\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTripleSwordStrike"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::multiset::MultiSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
}

fn solve_one_dim(pts: &[Point]) -> u32 {
    let max_x = pts.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = pts.iter().map(|p| p.y).max().unwrap() + 1;
    let mut by_x = vec![vec![]; max_x];
    let mut sum_by_x = vec![0; max_x];
    let mut sum_by_y = vec![0; max_y];
    for &p in pts.iter() {
        by_x[p.x].push(p);
        sum_by_x[p.x] += p.value;
        sum_by_y[p.y] += p.value;
    }
    let mut dp = vec![0; 4];
    for x in 0..max_x {
        for i in (0..3).rev() {
            let cur = dp[i];
            dp[i + 1].update_max(cur + sum_by_x[x]);
        }
    }
    let mut res = 0;
    for &x in dp.iter() {
        res.update_max(x);
    }

    let mut answers_by_y = MultiSet::new();
    for y in 0..max_y {
        if sum_by_y[y] != 0 {
            answers_by_y.insert(sum_by_y[y]);
        }
    }

    for x in 0..max_x {
        let mut candidates = vec![];
        for p in by_x[x].iter() {
            let y = p.y;
            assert!(sum_by_y[y] != 0);
            answers_by_y.remove(&sum_by_y[y]);

            candidates.push(sum_by_y[y] - p.value);
        }
        if let Some(&last) = answers_by_y.last() {
            candidates.push(last);
            answers_by_y.remove(&last);
            if let Some(&last2) = answers_by_y.last() {
                candidates.push(last2);
            }
            answers_by_y.insert(last);
        }
        for p in by_x[x].iter() {
            let y = p.y;
            assert!(sum_by_y[y] != 0);
            answers_by_y.insert(sum_by_y[y]);
        }
        candidates.sort();
        let mut cur_sum = sum_by_x[x];
        for _ in 0..2 {
            if candidates.len() != 0 {
                cur_sum += *candidates.last_exn();
                candidates.remove(candidates.len() - 1);
            }
        }
        res.update_max(cur_sum);
    }

    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let pts = gen_vec(n, |_| Point {
        x: input.read(),
        y: input.read(),
        value: input.read(),
    });
    let mut res = solve_one_dim(&pts);
    let rev_pts: Vec<_> = pts
        .into_iter()
        .map(|p| Point {
            x: p.y,
            y: p.x,
            value: p.value,
        })
        .collect();
    res.update_max(solve_one_dim(&rev_pts));
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
