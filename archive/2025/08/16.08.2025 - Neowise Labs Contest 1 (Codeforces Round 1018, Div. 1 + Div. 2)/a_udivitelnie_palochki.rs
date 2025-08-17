//{"name":"A. Удивительные палочки","group":"Codeforces - Neowise Labs Contest 1 (Codeforces Round 1018, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2096/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n<\n5\n<<><\n2\n>\n3\n<>\n7\n><>>><\n","output":"2 1\n4 3 2 5 1\n1 2\n2 1 3\n3 4 2 5 6 7 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AUdivitelniePalochki"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let s = input.string();
        let mut res = vec![n + 10];
        let mut left = n + 9;
        let mut right = n + 11;
        for i in 0..n - 1 {
            if s[i] == b'<' {
                res.push(left);
                left -= 1;
            } else {
                res.push(right);
                right += 1;
            }
        }
        let mn = res.iter().copied().min().unwrap();
        for i in 0..n {
            res[i] -= mn - 1;
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
    const PROBLEM_NAME: &str = "a_udivitelnie_palochki";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
