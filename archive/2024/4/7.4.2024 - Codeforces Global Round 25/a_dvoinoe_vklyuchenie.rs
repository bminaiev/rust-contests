//{"name":"A. Двойное включение","group":"Codeforces - Codeforces Global Round 25","url":"https://codeforces.com/contest/1951/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10\n1101010110\n10\n1001001110\n6\n000000\n1\n1\n12\n111111111111\n","output":"YES\nNO\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADvoinoeVklyuchenie"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut ok = true;
    let mut cnt_ones = 0;
    for &c in s.iter() {
        if c == b'1' {
            cnt_ones += 1;
        }
    }
    if cnt_ones % 2 != 0 {
        ok = false;
    } else {
        if cnt_ones == 2 {
            for w in s.windows(2) {
                if w[0] == b'1' && w[1] == b'1' {
                    ok = false;
                }
            }
        }
    }
    if ok {
        out.println("YES");
    } else {
        out.println("NO");
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
    const PROBLEM_NAME: &str = "a_dvoinoe_vklyuchenie";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
