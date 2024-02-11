//{"name":"L. Ordering Value","group":"Codeforces - Testing Constructor Cup","url":"https://codeforces.com/gym/503340/problem/L","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n2 1 1 3\n4\n7 5 3 4\n4\n2 1 4 3\n6\n2 3 3 1 1 2\n","output":"1\n3\n2\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LOrderingValue"}}}

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let mut sorted_a = a.clone();
    sorted_a.sort();
    let mut g = HashMap::<usize, Vec<usize>>::new();
    let mut cnt_edges = 0;
    for i in 0..n {
        let from = sorted_a[i];
        let to = a[i];
        if from != to {
            g.entry(from).or_default().push(to);
            g.entry(to).or_default().push(from);
            cnt_edges += 1;
        }
    }
    let mut seen = HashSet::new();
    let mut res = cnt_edges;
    for &start in g.keys() {
        if seen.contains(&start) {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            seen.insert(node);
            if let Some(neighs) = g.get(&node) {
                for &neigh in neighs {
                    if !seen.contains(&neigh) {
                        queue.push_back(neigh);
                    }
                }
            }
        }
        res -= 1;
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "l_ordering_value";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
