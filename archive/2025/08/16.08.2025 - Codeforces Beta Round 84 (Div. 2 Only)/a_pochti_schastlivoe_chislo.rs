//{"name":"A. Почти счастливое число","group":"Codeforces - Codeforces Beta Round 84 (Div. 2 Only)","url":"https://codeforces.com/problemset/problem/110/A","interactive":false,"timeLimit":2000,"tests":[{"input":"40047\n","output":"NO\n"},{"input":"7747774\n","output":"YES\n"},{"input":"1000000000000000000\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APochtiSchastlivoeChislo"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let s = input.string();
    let mut cnt = 0;
    for &c in s.iter() {
        if c == b'7' || c == b'4' {
            cnt += 1;
        }
    }
    if cnt == 4 || cnt == 7 {
        out.println("YES");
    } else {
        out.println("NO");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_pochti_schastlivoe_chislo";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
