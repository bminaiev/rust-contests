//{"name":"A. Квинтомания","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n2\n114 109\n2\n17 10\n3\n76 83 88\n8\n38 45 38 80 85 92 99 106\n5\n63 58 65 58 65\n8\n117 124 48 53 48 43 54 49\n5\n95 102 107 114 121\n10\n72 77 82 75 70 75 68 75 68 75\n","output":"YES\nYES\nYES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKvintomaniya"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n);
        let ok = a.windows(2).all(|w| {
            let d = (w[0] - w[1]).abs();
            d == 5 || d == 7
        });
        out.println(if ok { "YES" } else { "NO" });
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_kvintomaniya";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
