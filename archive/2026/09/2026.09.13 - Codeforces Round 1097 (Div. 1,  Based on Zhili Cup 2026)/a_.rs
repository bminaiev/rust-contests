#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let ss = [input.string(), input.string()];
        let mut balances = [0, 0];
        let mut ok = true;
        for i in 0..n {
            let cnt_open = (ss[0][i] == b'(') as i32 + (ss[1][i] == b'(') as i32;
            if cnt_open == 0 {
                balances[0] -= 1;
                balances[1] -= 1;
            } else if cnt_open == 1 {
                balances[0] += 1;
                balances[1] -= 1;
            } else {
                balances[0] += 1;
                balances[1] += 1;
            }
            balances.sort();
            if balances[0] < 0 {
                ok = false;
            }
        }
        if ok && balances[1] == 0 {
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
