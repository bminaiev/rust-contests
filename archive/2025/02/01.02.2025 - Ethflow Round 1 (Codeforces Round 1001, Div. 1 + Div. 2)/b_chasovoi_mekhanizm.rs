//{"name":"B. Часовой механизм","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"5\n2\n4 10\n2\n2 2\n3\n4 10 5\n3\n5 3 5\n5\n12 13 25 17 30\n","output":"YES\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BChasovoiMekhanizm"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);
        let mut ok = true;
        for i in 0..n {
            if a[i] == 1 {
                ok = false;
            }
            let len = (2 * i).max(2 * (n - 1 - i));
            if a[i] <= len {
                ok = false;
            }
        }
        if ok {
            out.println("YES");
        } else {
            out.println("NO");
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
    const PROBLEM_NAME: &str = "b_chasovoi_mekhanizm";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
