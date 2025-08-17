//{"name":"B. Перетасовка стопок","group":"Codeforces - Order Capital Round 1 (Codeforces Round 1038, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2122/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n1 3 1 2\n1 1 1 2\n3\n2 0 2 2\n0 1 1 0\n1 1 0 0\n3\n1 2 1 2\n3 4 3 4\n0 0 0 0\n","output":"2\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPeretasovkaStopok"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut res = 0;
        for _ in 0..n {
            let a = input.i64();
            let b = input.i64();
            let c = input.i64();
            let d = input.i64();
            if b > d {
                res += (a + b) - d;
            } else if a > c {
                res += a - c;
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_peretasovka_stopok";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
