//{"name":"B. Стартап","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 3\n2 6\n2 7\n1 15\n1 3\n2 6\n2 7\n1 15\n6 2\n1 7\n2 5\n190000 1\n1 1000\n","output":"28\n15\n12\n1000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BStartap"}}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let k = input.usize();
        let n = input.usize();
        let mut hm = HashMap::<i32, i64>::new();
        for _ in 0..n {
            let brand = input.i32();
            let cost = input.i64();
            *hm.entry(brand).or_default() += cost;
        }
        let mut all_costs: Vec<_> = hm.values().copied().collect();
        all_costs.sort();
        all_costs.reverse();
        all_costs.truncate(k);
        let ans: i64 = all_costs.iter().sum();
        out.println(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_startap";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
