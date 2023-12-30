//{"name":"A. 2023","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2 2\n5 2\n3 1\n7 17 7\n4 2\n1 289 1 1\n3 1\n7 17 17\n1 1\n289\n1 1\n2023\n1 3\n1\n","output":"NO\nNO\nYES\n7 1\nYES\n1\nYES\n7\nYES\n1\nYES\n7 17 17\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A2023"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let b = input.vec::<i64>(n);
    let mut mul = 1;
    for &x in b.iter() {
        mul *= x;
    }
    if 2023 % mul == 0 {
        out.println("YES");
        for _ in 0..k - 1 {
            out.print("1 ");
        }
        out.println(2023 / mul);
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
    const PROBLEM_NAME: &str = "a2023";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
