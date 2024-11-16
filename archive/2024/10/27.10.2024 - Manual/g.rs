//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::collections::VecDeque;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve_case(good: &[bool], g: &[Vec<usize>]) -> Option<Vec<Vec<usize>>> {
    let n = good.len();
    let mut root = n;
    for i in 0..n {
        if good[i] {
            root = i;
        }
    }
    if root == n {
        return None;
    }
    let mut res = vec![];
    let mut seen = vec![false; n];
    seen[root] = true;
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while let Some(v) = queue.pop_front() {
        let mut cur_step = vec![v + 1];
        for &u in &g[v] {
            if !seen[u] {
                if good[u] {
                    queue.push_back(u);
                }
                cur_step.push(u + 1);
                seen[u] = true;
            }
        }

        let cnt_events = cur_step.len() - 1;
        if cnt_events != 0 {
            cur_step.insert(1, cnt_events);
            res.push(cur_step);
        }
    }
    // for i in 0..n {
    //     if seen[i] && good[i] {
    //         for to in &g[i] {
    //             if !seen[*to] {
    //                 res.push(vec![i + 1, 1, *to + 1]);
    //                 seen[*to] = true;
    //             }
    //         }
    //     }
    // }
    for i in 0..n {
        if !seen[i] {
            return None;
        }
    }
    let cnt_events = res.len();
    res.insert(0, vec![cnt_events]);
    Some(res)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize();
    let mut good = vec![true; n];
    for _ in 0..k {
        let x = input.usize() - 1;
        good[x] = false;
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let x = input.usize() - 1;
        let y = input.usize() - 1;
        g[x].push(y);
        g[y].push(x);
    }
    if let Some(res) = solve_case(&good, &g) {
        out.println("Yes");
        for r in res.into_iter() {
            out.println(r);
        }
    } else {
        out.println("No");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "g";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
