#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = input.vec::<usize>(n);
        let mut to_swap = vec![];
        for i in 0..n {
            if a[i] != i + 1 {
                to_swap.push(i);
            }
        }
        for i in 0..to_swap.len() / 2 {
            let x = to_swap[i];
            let y = to_swap[to_swap.len() - 1 - i];
            a.swap(x, y);
        }
        if a.is_sorted() {
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
    const PROBLEM_NAME: &str = "a_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
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
