//{"name":"B - Stolen Necklace","group":"AtCoder - AtCoder Regular Contest 215","url":"https://atcoder.jp/contests/arc215/tasks/arc215_b","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3\n1 2 2 3 3 1\n5\n1 2 3 4 5 5 4 3 2 1\n","output":"3\n1 2 4\n1\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n * 2).sub_from_all(1);
        let mut divs = vec![];
        let mut seen = vec![vec![false; n]; 2];
        let mut cur = 0;
        for i in 0..2 * n {
            if seen[cur][a[i]] {
                divs.push(i);
                cur = 1 - cur;
            }
            seen[cur][a[i]] = true;
        }
        assert!(divs.len() <= n);
        out.println(divs.len());
        out.println(divs);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_stolen_necklace";
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
