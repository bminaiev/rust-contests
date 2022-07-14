//{"name":"F. Точки","group":"Codeforces - Educational Codeforces Round 131 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1701/problem/F","interactive":false,"timeLimit":6500,"tests":[{"input":"7 5\n8 5 3 2 1 5 6\n","output":"0\n0\n1\n2\n5\n1\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTochki"}}}

use std::hash::Hash;
use std::ops::Not;

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[target_feature(enable = "avx2")]
pub unsafe fn calc_res(alive: &[i32], cnt: &[i32]) -> i64 {
    cnt.iter()
        .zip(alive.iter())
        .map(|(&v, &alive)| (alive & v) as i64)
        .sum()
}

#[target_feature(enable = "avx2")]
pub unsafe fn add_const(arr: &mut [i32], c: i32) {
    for x in arr.iter_mut() {
        *x += c;
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn add_const2(arr: &mut [i64], c: i64) {
    for x in arr.iter_mut() {
        *x += c;
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn calc_res2(alive: &[i32], cnt: &[i32], delta0: i32) -> i64 {
    cnt.iter()
        .zip(alive.iter())
        .map(|(&cnt, &alive)| (alive & (cnt + delta0)) as i64)
        .sum()
}

fn real_sol(queries: &[usize], seg_start: &[usize]) -> Vec<i64> {
    let max_coord = *queries.iter().max().unwrap();
    let mut alive = vec![0i32; max_coord + 1];
    let mut cnt = vec![0; max_coord + 1];

    let mut results = vec![];
    let mut res = 0;
    for &query in queries.iter() {
        alive[query] = alive[query].not();

        let cc = cnt[query] as i64;
        if alive[query] != 1 {
            res += cc * (cc - 1);
            res += unsafe {
                calc_res(
                    &alive[seg_start[query]..query],
                    &cnt[seg_start[query]..query],
                )
            } * 2;
            unsafe {
                add_const(&mut cnt[seg_start[query]..query], 1);
            }
        } else {
            res -= cc * (cc - 1);
            unsafe {
                add_const(&mut cnt[seg_start[query]..query], -1);
            }
            res -= unsafe {
                calc_res(
                    &alive[seg_start[query]..query],
                    &cnt[seg_start[query]..query],
                )
            } * 2;
        }
        results.push(res / 2);
    }
    results
}

fn baseline(queries: &[usize], seg_start: &[usize]) -> Vec<i64> {
    let max_coord = *queries.iter().max().unwrap();
    let mut alive = vec![false; max_coord + 1];
    let mut cnt = vec![0i64; max_coord + 1];

    let mut results = vec![];
    for &query in queries.iter() {
        alive[query] = !alive[query];
        let delta = if alive[query] { 1 } else { -1 };
        for c in cnt[seg_start[query]..query].iter_mut() {
            *c += delta;
        }
        let res = cnt
            .iter()
            .zip(alive.iter())
            .map(|(&cnt, &alive)| if alive { cnt * (cnt - 1) } else { 0 })
            .sum::<i64>();
        results.push(res / 2);
    }
    results
}

fn ver1(queries: &[usize], seg_start: &[usize]) -> Vec<i64> {
    let max_coord = *queries.iter().max().unwrap();
    let mut alive = vec![0i32; max_coord + 1];
    let mut cnt = vec![0i32; max_coord + 1];

    let mut results = vec![];
    let mut res = 0;
    for &query in queries.iter() {
        alive[query] = alive[query].not();
        let delta0 = if alive[query] != 0 { 0 } else { -1 };
        let delta = if alive[query] != 0 { 1 } else { -1 };
        res += unsafe {
            calc_res2(
                &alive[seg_start[query]..query],
                &cnt[seg_start[query]..query],
                delta0,
            )
        } * 2
            * delta;
        res += delta * (cnt[query] as i64) * (cnt[query] as i64 - 1);
        unsafe {
            add_const(&mut cnt[seg_start[query]..query], delta as i32);
        }
        results.push(res / 2);
    }
    results
}

fn solve_case(d: i32, queries: Vec<i32>) -> Vec<i64> {
    let mut coords = queries.sorted();
    coords.dedup();
    let queries: Vec<usize> = queries
        .into_iter()
        .map(|val| coords.binary_search(&val).unwrap())
        .collect();
    let from: Vec<_> = (0..coords.len())
        .map(|pos| binary_search_first_true(0..pos, |p| coords[p] + d >= coords[pos]))
        .collect();
    ver1(&queries, &from)
}

fn stress() {
    let d = 1_000_000;
    let queries: Vec<_> = (0..200_000).collect();
    let res = solve_case(d, queries);
    let hash: i64 = res.iter().sum();
    assert_eq!(hash, -7120976296504856464);
}

fn solve(input: &mut Input, _test_case: usize) {
    // let q = input.usize();
    // let d = input.i32();
    // let queries = input.vec::<i32>(q);
    // out_line!(solve_case(d, queries));
    stress();
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
    // tester::run_tests();
    tester::run_stress(stress);
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
