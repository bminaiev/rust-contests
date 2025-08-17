//{"name":"A. Скупая сетка","group":"Codeforces - Order Capital Round 1 (Codeforces Round 1038, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2122/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 3\n1 2\n","output":"YES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASkupayaSetka"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let mn = n.min(m);
        let mx = n.max(m);
        if mn == 1 || mx == 2 {
            out.println("NO");
        } else {
            out.println("YES");
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
    const PROBLEM_NAME: &str = "a_skupaya_setka";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
