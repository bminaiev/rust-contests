//{"name":"A - Neq Number","group":"AtCoder - AtCoder Regular Contest 173","url":"https://atcoder.jp/contests/arc173/tasks/arc173_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n25\n148\n998244353\n","output":"27\n173\n2506230721\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANeqNumber"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let mut n = input.u64() - 1;
    let mut powers = vec![1];
    for _ in 0..18 {
        powers.push(powers.last().unwrap() * 9);
    }
    for len in 1.. {
        if n < powers[len] {
            let mut last_digit = 0;
            for pos in 0..len {
                let mut digit = n / powers[len - pos - 1];
                n -= digit * powers[len - pos - 1];
                if digit >= last_digit {
                    digit += 1;
                }
                out.print(digit);
                last_digit = digit;
            }
            out.println("");
            return;
        } else {
            n -= powers[len];
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_neq_number";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
