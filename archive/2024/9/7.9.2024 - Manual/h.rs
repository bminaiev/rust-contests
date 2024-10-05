//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use std::io::Write;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    const MAX: usize = 1_000_010;
    let mut splits = vec![0, 0, 0];
    let mut sum_len = vec![0, 0, 2];
    let mut queries = vec![0, 0, 1];
    for n in 3..=MAX {
        let mut split = splits[n - 1].max(1);
        while split + 1 < n && sum_len[split + 1] + n <= 3 * n {
            let cur_queries = (1 + queries[split]).max(2 + queries[n - split]);
            let next_queries = (1 + queries[split + 1]).max(2 + queries[n - split - 1]);
            if next_queries > cur_queries {
                break;
            }
            split += 1;
        }
        splits.push(split);
        let left = n - split;
        let mut next_sum_len = sum_len[split] + n;
        next_sum_len = next_sum_len.max(n + split + sum_len[left]);
        sum_len.push(next_sum_len);
        let mut next_queries = 1 + queries[split];
        if queries[split] == 0 {
            next_queries = 2;
        }
        next_queries = next_queries.max(2 + queries[left]);
        queries.push(next_queries);
        let max_allowed_queries = ((n as f64).log2() * 1.5).ceil() as usize;
        assert!(next_sum_len <= 3 * n);
        assert!(next_queries <= max_allowed_queries);
    }
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut from = 1;
        let mut to = n;
        let mut second_max = None;
        while from < to {
            // dbg!(from, to);
            if second_max.is_none() {
                out.println(format!("? {} {}", from, to));
                out.flush();
                second_max = Some(input.usize());
            }
            let n = to - from + 1;
            if n == 2 {
                if second_max.unwrap() == from {
                    from += 1;
                } else {
                    to -= 1;
                }
            } else {
                let split = splits[n];
                if from + split > second_max.unwrap() {
                    let mid = from + split - 1;
                    out.println(format!("? {} {}", from, mid));
                    out.flush();
                    let pos = input.usize();
                    if pos == second_max.unwrap() {
                        to = mid;
                    } else {
                        from = mid + 1;
                        second_max = None;
                    }
                } else {
                    let mid = to - split + 1;
                    out.println(format!("? {} {}", mid, to));
                    out.flush();
                    let pos = input.usize();
                    if pos == second_max.unwrap() {
                        from = mid;
                    } else {
                        to = mid - 1;
                        second_max = None;
                    }
                }
            }
        }
        out.println(format!("! {}", from));
        out.flush();
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "h";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    run_locally(run);
}
//END MAIN
