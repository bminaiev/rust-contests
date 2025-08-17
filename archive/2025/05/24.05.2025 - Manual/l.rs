//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use std::collections::VecDeque;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let next = input.vec::<usize>(n).sub_from_all(1);
    let mut prevs = vec![vec![]; n];
    for i in 0..n {
        prevs[next[i]].push(i);
    }
    let mut must_be_ok = vec![false; n];
    let mut bad = vec![false; n];
    let mut cnt_in = vec![0; n];
    for i in 0..n {
        cnt_in[next[i]] += 1;
    }
    let mut seen = vec![false; n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        if cnt_in[i] == 0 {
            queue.push_back(i);
        }
    }
    while let Some(v) = queue.pop_front() {
        seen[v] = true;
        if !must_be_ok[v] {
            bad[v] = true;
            must_be_ok[next[v]] = true;
        }
        cnt_in[next[v]] -= 1;
        if cnt_in[next[v]] == 0 {
            queue.push_back(next[v]);
        }
    }
    let mut good_queue = VecDeque::new();
    for i in 0..n {
        if must_be_ok[i] && next[i] != i {
            good_queue.push_back(i);
        }
    }
    while let Some(v) = good_queue.pop_front() {
        if seen[v] {
            continue;
        }
        seen[v] = true;
        for &u in &prevs[v] {
            if !must_be_ok[u] && !bad[u] {
                bad[u] = true;
                seen[u] = true;
                for p in prevs[u].iter() {
                    if !seen[*p] {
                        good_queue.push_back(*p);
                        must_be_ok[*p] = true;
                    }
                }
            }
        }
    }
    for root in 0..n {
        if seen[root] {
            continue;
        }
        let mut v = root;
        let mut cycle = vec![];
        loop {
            seen[v] = true;
            cycle.push(v);
            v = next[v];
            if seen[v] {
                break;
            }
        }
        for i in (0..cycle.len()).step_by(2) {
            bad[cycle[i]] = true;
        }
        if cycle.len() % 2 == 1 {
            bad[cycle[0]] = false;
        }
    }

    let mut total_bad = 0;
    for i in 0..n {
        if bad[i] {
            total_bad += 1;
            assert!(!bad[next[i]]);
        }
    }
    out.println(total_bad);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "l";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
