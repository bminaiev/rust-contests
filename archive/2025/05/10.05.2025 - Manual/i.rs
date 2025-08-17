//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let s1 = input
            .string_as_string()
            .split(".")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let s2 = input
            .string_as_string()
            .split(".")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        assert!(s1.len() == s2.len());
        if s1 > s2 {
            out.println("A");
        } else if s1 < s2 {
            out.println("B");
        } else {
            out.println("Equal");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
