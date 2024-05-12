//{"name":"A - Exchange","group":"AtCoder - AtCoder Regular Contest 177","url":"https://atcoder.jp/contests/arc177/tasks/arc177_a","interactive":false,"timeLimit":2000,"tests":[{"input":"0 0 6 3 4 1\n3\n700 250 160\n","output":"Yes\n"},{"input":"0 0 0 2 4 0\n3\n100 200 300\n","output":"No\n"},{"input":"0 0 0 0 8 8\n1\n250\n","output":"No\n"},{"input":"20 5 9 7 10 6\n5\n177 177 177 177 177\n","output":"Yes\n"},{"input":"17 5 9 7 10 6\n5\n177 177 177 177 177\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AExchange"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    const C: usize = 6;
    let muls = [1, 5, 10, 50, 100, 500];
    let mut a = input.vec::<i64>(C);
    let n = input.usize();
    let mut ok = true;
    for _ in 0..n {
        let mut x = input.i64();
        for i in (0..C).rev() {
            let t = std::cmp::min(x / muls[i], a[i]);
            x -= t * muls[i];
            a[i] -= t;
        }
        if x != 0 {
            ok = false;
        }
    }
    out.println(if ok { "Yes" } else { "No" });
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_exchange";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
