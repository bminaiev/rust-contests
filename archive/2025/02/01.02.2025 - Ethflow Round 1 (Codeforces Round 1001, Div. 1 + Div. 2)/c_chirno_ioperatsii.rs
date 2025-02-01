//{"name":"C. Чирно и операции","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1\n-1000\n2\n5 -3\n2\n1000 1\n9\n9 7 9 -9 9 -8 7 -8 9\n11\n678 201 340 444 453 922 128 987 127 752 0\n","output":"-1000\n8\n1001\n2056\n269891\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CChirnoIOperatsii"}}}

use std::collections::{HashSet, VecDeque};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n);
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(a.clone());
        seen.insert(a.clone());
        let mut max_sum = i64::MIN;
        while let Some(cur) = queue.pop_front() {
            let sum = cur.iter().sum::<i64>();
            max_sum = max_sum.max(sum);
            {
                let mut next = cur.clone();
                next.reverse();
                if seen.insert(next.clone()) {
                    queue.push_back(next);
                }
            }
            if cur.len() > 1 {
                let mut deltas = vec![];
                for i in 0..cur.len() - 1 {
                    deltas.push(cur[i] - cur[i + 1]);
                }
                if seen.insert(deltas.clone()) {
                    queue.push_back(deltas);
                }
            }
        }
        out.println(max_sum);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c_chirno_ioperatsii";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
