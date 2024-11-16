//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize();
    let mut hm = HashMap::<Vec<u8>, Vec<(usize, usize)>>::new();
    for i in 0..n {
        let s = input.string();
        for j in 0..m {
            let cur = s[j * k..(j + 1) * k].to_vec();
            hm.entry(cur).or_default().push((i + 1, j + 1));
        }
    }
    for (key, val) in hm {
        if val.len() == 1 {
            out.println(val);
            return;
        }
    }
    panic!()
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
