//{"name":"H. Кейген 3","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/H","interactive":false,"timeLimit":4000,"tests":[{"input":"6 3\n","output":"9\n1 4 5 6 3 2\n6 5 4 3 2 1\n1 2 4 5 6 3\n1 2 5 6 4 3\n1 3 4 6 5 2\n1 5 6 4 3 2\n3 5 6 4 2 1\n1 3 6 5 4 2\n2 6 5 4 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::HashSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

const NEED: usize = 2_000;

fn solve_case(n: usize, need_cycles: usize) -> Vec<Vec<usize>> {
    let mut res = vec![];
    for mask in 0..(1 << n) {
        let mut left = vec![];
        let mut right = vec![];
        for i in 0..n {
            if (mask & (1 << i)) == 0 {
                left.push(i);
            } else {
                right.push(i);
            }
        }
        left.sort();
        right.sort();
        right.reverse();
        let mut perm = left;
        perm.extend(right);
        let mut visited = vec![false; n];
        let mut cycles = 0;
        for i in 0..n {
            if !visited[i] {
                cycles += 1;
                let mut cur = i;
                while !visited[cur] {
                    visited[cur] = true;
                    cur = perm[cur];
                }
            }
        }
        if cycles == need_cycles {
            res.push(perm);
        }
    }
    res.sort();
    res.dedup();
    res
}

fn solve_case_new(n: usize, need_cycles: usize) -> Vec<Vec<usize>> {
    let mut answers = vec![HashSet::new(); n + 1];
    let mut rnd = Random::new(123123);
    const LAST: usize = 20;

    let mut start = need_cycles - 1;
    if start + LAST > n {
        if n >= LAST {
            start = n - LAST;
        } else {
            start = 0;
        }
    }

    for mask in 0..1 << LAST {
        let mut left = vec![];
        let mut right = vec![];
        for i in 0..n {
            let go_left = if i < start {
                true
            } else if i < start + LAST {
                mask & (1 << (i - start)) == 0
            } else {
                rnd.gen_bool()
            };

            if go_left {
                left.push(i);
            } else {
                right.push(i);
            }
        }
        left.sort();
        right.sort();
        right.reverse();
        let mut perm = left;
        perm.extend(right);
        let mut visited = vec![false; n];
        let mut cycles = 0;
        for i in 0..n {
            if !visited[i] {
                cycles += 1;
                let mut cur = i;
                while !visited[cur] {
                    visited[cur] = true;
                    cur = perm[cur];
                }
            }
        }
        if answers[cycles].len() >= NEED {
            continue;
        }
        answers[cycles].insert(perm);
    }
    if n >= 20 && n - need_cycles >= 10 {
        assert_eq!(answers[need_cycles].len(), NEED);
    }
    answers[need_cycles].iter().cloned().collect()
}

fn stress() {
    for n in 1..20 {
        for need_cycles in 1..=n {
            dbg!(n, need_cycles);
            let res1 = solve_case(n, need_cycles);
            let res2 = solve_case_new(n, need_cycles);
            let mut res1_sorted = res1.clone();
            res1_sorted.sort();
            let mut res2_sorted = res2.clone();
            res2_sorted.sort();
            assert_eq!(
                res1_sorted, res2_sorted,
                "n={}, need_cycles={}",
                n, need_cycles
            );
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let cnt_cycles = input.usize();
    let mut perms = solve_case_new(n, cnt_cycles);
    perms.truncate(2000);
    out.println(perms.len());
    for p in perms {
        let p1: Vec<usize> = p.iter().map(|x| x + 1).collect();
        out.println(p1);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "h_3";
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
