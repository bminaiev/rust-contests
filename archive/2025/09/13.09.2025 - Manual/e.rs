//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn mul(a: &[i64], b: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![i64::MAX; n];
    for i in 0..n {
        for j in 0..n {
            let k = (i + j) % n;
            res[k] = res[k].min(a[i] + b[j]);
        }
    }
    res
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    value: i64,
    pos: usize,
}

fn mul_fast(a: &[i64], b: &[i64], max_n: usize) -> Vec<i64> {
    let n = a.len();
    let mut a_items = a
        .iter()
        .enumerate()
        .map(|(i, &v)| Item { value: v, pos: i })
        .collect::<Vec<_>>();
    let mut b_items = b
        .iter()
        .enumerate()
        .map(|(i, &v)| Item { value: v, pos: i })
        .collect::<Vec<_>>();
    a_items.sort();
    b_items.sort();
    a_items.truncate(max_n);
    b_items.truncate(max_n);
    // dbg!(a_items.len(), b_items.len(), max_n);
    let mut res = vec![i64::MAX; n];
    for a in a_items.iter() {
        for b in b_items.iter() {
            let mut k = (a.pos + b.pos) % n;
            res[k] = res[k].min(a.value + b.value);
        }
    }
    res
}

fn solve_case(a: &[i64], max_n: usize) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![];
    let mut cur_pw = a.to_vec();
    let mut need = n;
    for i in 0..n {
        if (need & (1 << i)) != 0 {
            if res.is_empty() {
                res = cur_pw.clone();
            } else {
                res = mul_fast(&res, &cur_pw, max_n);
            }
            need ^= 1 << i;
        }
        if need == 0 {
            break;
        }
        cur_pw = mul_fast(&cur_pw, &cur_pw, max_n);
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let a = input.vec(n);
    let res = solve_faster(&a, 150);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

fn solve_faster(a: &[i64], max_n: usize) -> Vec<i64> {
    let n = a.len();
    let min_pos = a.iter().enumerate().min_by_key(|(_, &v)| v).unwrap().0;
    let min_value = a[min_pos] * n as i64;
    let mut res = vec![i64::MAX / 2; n];
    res[0] = min_value;
    let mut a_items = a
        .iter()
        .enumerate()
        .map(|(i, &v)| Item {
            value: (v - a[min_pos]),
            pos: (i + n - min_pos) % n,
        })
        .collect::<Vec<_>>();
    a_items.sort();
    a_items.truncate(max_n);
    let mut pq = BinaryHeap::new();
    pq.push(Item {
        value: -min_value,
        pos: 0,
    });
    while let Some(item) = pq.pop() {
        let i = item.pos;
        let value = -item.value;
        if value > res[i] {
            continue;
        }
        for item in a_items.iter() {
            let k = (i + item.pos) % n;
            let next_value = value + item.value;
            if next_value < res[k] {
                res[k] = next_value;
                pq.push(Item {
                    value: -next_value,
                    pos: k,
                });
            }
        }
    }
    // loop {
    //     let mut changed = false;
    //     for i in 0..n {
    //         for item in a_items.iter() {
    //             let k = (i + item.pos) % n;
    //             let next_value = res[i] + item.value;
    //             if next_value < res[k] {
    //                 res[k] = next_value;
    //                 changed = true;
    //             }
    //         }
    //     }
    //     if !changed {
    //         break;
    //     }
    // }
    res
}

fn stress() {
    let n = 500_000;
    const MAX_N: usize = 100;
    for it in 100.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_V: i64 = 1_000_000_000;
        let a = rnd.gen_vec(n, 1..MAX_V);
        let slow_mul = solve_faster(&a, MAX_N * 2);
        let start = Instant::now();
        let fast_mul = solve_faster(&a, MAX_N);
        let min_val = *fast_mul.iter().min().unwrap();
        assert!(fast_mul[0] == min_val);
        dbg!(start.elapsed());
        if slow_mul != fast_mul {
            let mut cnt_diff = 0;
            for i in 0..n {
                if slow_mul[i] != fast_mul[i] {
                    cnt_diff += 1;
                    // dbg!(i, slow_mul[i], fast_mul[i]);
                }
            }
            dbg!(cnt_diff);
            assert!(false);
        }
    }
    // let res = solve_case(&a);
    // let mut hs = HashSet::new();
    // for x in res.iter() {
    //     hs.insert(x);
    //     dbg!(x);
    // }
    // dbg!(hs.len());
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
