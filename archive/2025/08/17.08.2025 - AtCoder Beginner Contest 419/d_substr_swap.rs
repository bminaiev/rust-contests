//{"name":"D - Substr Swap","group":"AtCoder - AtCoder Beginner Contest 419","url":"https://atcoder.jp/contests/abc419/tasks/abc419_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\napple\nlemon\n2 4\n1 5\n5 5\n","output":"lpple\n"},{"input":"10 5\nlemwrbogje\nomsjbfggme\n5 8\n4 8\n1 3\n6 6\n1 4\n","output":"lemwrfogje\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSubstrSwap"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let s = input.string();
    let t = input.string();
    let mut xor = vec![0; n + 1];
    for _ in 0..m {
        let l = input.usize() - 1;
        let r = input.usize();
        xor[l] ^= 1;
        xor[r] ^= 1;
    }
    let mut my_xor = 0;
    let mut res = vec![];
    for i in 0..n {
        my_xor ^= xor[i];
        if my_xor == 1 {
            res.push(t[i]);
        } else {
            res.push(s[i]);
        }
    }
    out.println(res.iter().map(|x| *x as char).collect::<String>());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_substr_swap";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
