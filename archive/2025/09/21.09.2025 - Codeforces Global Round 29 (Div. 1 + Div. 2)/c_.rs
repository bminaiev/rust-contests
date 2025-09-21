//{"name":"C. Кролики","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n4\n0100\n3\n000\n8\n11011011\n5\n00100\n1\n1\n5\n01011\n2\n01\n7\n0101011\n7\n1101010\n5\n11001\n4\n1101\n9\n001101100\n","output":"YES\nYES\nNO\nYES\nYES\nYES\nYES\nYES\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let s = input.string();
        let mut ok = true;
        let mut it = 0;
        while it < n {
            if s[it] == b'1' {
                it += 1;
                continue;
            }
            let mut good_zero = false;
            if it == 0 || s[it - 1] == b'0' {
                good_zero = true;
            }
            let mut last_zero = it;
            while last_zero + 2 < n && s[last_zero + 1] == b'1' && s[last_zero + 2] == b'0' {
                last_zero += 2;
            }
            let cnt_zeros = (last_zero - it) / 2 + 1;
            if cnt_zeros % 2 == 1 && last_zero + 1 < n && s[last_zero + 1] == b'1' && !good_zero {
                ok = false;
                break;
            }
            it = last_zero + 1;
        }
        if ok {
            out.println("YES");
        } else {
            out.println("NO");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
