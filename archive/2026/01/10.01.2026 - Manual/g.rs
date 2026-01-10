//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::cmp::Reverse;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::two_min::TwoMin;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Score {
    cnt_seg: usize,
    sum_len: i32,
}

#[derive(Copy, Clone)]
struct Event {
    id: usize,
    next: usize,
    len: i32,
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut a = vec![];
    let mut all_pos = vec![];
    for _ in 0..n {
        let seg = input.vec::<i32>(3);
        for x in &seg {
            all_pos.push(*x);
        }
        a.push(seg);
    }
    all_pos.sort();
    all_pos.dedup();
    let mut events = vec![vec![]; all_pos.len()];
    for (id, seg) in a.iter().enumerate() {
        for l_idx in 0..3 {
            for r_idx in l_idx + 1..3 {
                let l = seg[l_idx];
                let r = seg[r_idx];
                let l_pos = all_pos.binary_search(&l).unwrap();
                let r_pos = all_pos.binary_search(&r).unwrap();
                let len = r - l;
                events[l_pos].push(Event {
                    id,
                    next: r_pos,
                    len,
                });
            }
        }
    }
    let tw = TwoMin::new(
        n + 2,
        Reverse(Score {
            cnt_seg: 0,
            sum_len: 0,
        }),
    );
    let mut dp = vec![tw; all_pos.len()];
    dp[0].add(
        n + 1,
        Reverse(Score {
            cnt_seg: 0,
            sum_len: 0,
        }),
    );
    let mut res = 0;
    for i in 0..dp.len() {
        if i > 0 {
            let prev = dp[i - 1].clone();
            dp[i].merge(&prev);
        }
        if let Some(mx) = dp[i].get_value_by_not_id(n + 2) {
            if mx.0.cnt_seg == n {
                res = res.max(mx.0.sum_len);
            }
        }
        for e in events[i].iter() {
            if let Some(mx) = dp[i].get_value_by_not_id(e.id) {
                let next_pos = e.next;
                dp[next_pos].add(
                    e.id,
                    Reverse(Score {
                        cnt_seg: mx.0.cnt_seg + 1,
                        sum_len: mx.0.sum_len + e.len,
                    }),
                );
            }
        }
    }
    if res == 0 {
        out.println(-1);
    } else {
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "g";
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
