#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::two_sat::TwoSat;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        // ts[i] == true -> -1
        let mut ts = TwoSat::new(n);
        for _ in 0..m {
            let ty = input.usize();
            let x = input.usize() - 1;
            let y = input.usize() - 1;
            if ty == 1 {
                // sum >= 0
                ts.add_edge(x, true, x, false);
                ts.add_edge(y, true, y, false);
            } else {
                // sum < 0
                ts.add_edge(x, false, y, true);
                ts.add_edge(y, false, x, true);
            }
        }
        if let Some(ans) = ts.find_solution() {
            out.println("YES");
            let mut res = vec![];
            for i in 0..n {
                if ans[i] {
                    res.push(-1);
                } else {
                    res.push(0);
                }
            }
            out.println(res);
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
    const PROBLEM_NAME: &str = "d2_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
