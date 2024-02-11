//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::io::Write;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut ask = |p: usize, q: usize| -> bool {
            let p = (p % n) + 1;
            let q = (q % n) + 1;
            out.println(format!("? {} {}", p, q));
            out.flush();
            input.read::<String>().as_str() == "1"
        };
        const CHAIN: usize = 1;
        const STAR: usize = 2;
        let mut res = CHAIN;
        for start in (0..=n).step_by(2) {
            if ask(start, start + 1) {
                if ask(start, start + 2) {
                    if ask(start, start + 3) {
                        res = STAR;
                    } else {
                        res = CHAIN;
                    }
                } else {
                    if ask(start + 1, start + 2) {
                        if ask(start + 1, start + 3) {
                            res = STAR;
                        } else {
                            res = CHAIN;
                        }
                    } else {
                        res = CHAIN;
                    }
                }
                break;
            }
        }
        out.println(format!("! {}", res));
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
    const PROBLEM_NAME: &str = "j";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    run_locally(run);
}
//END MAIN
