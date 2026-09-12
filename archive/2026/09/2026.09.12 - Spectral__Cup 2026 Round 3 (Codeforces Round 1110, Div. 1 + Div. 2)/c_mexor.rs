#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let need = input.i64() ^ (n as i64);
        let mut perm = vec![];
        let mut ok = true;
        let mut used = vec![false; n];
        for bit in (0..33).rev() {
            if (need >> bit) & 1 == 1 {
                let cur = 1 << bit;
                if cur >= n {
                    ok = false;
                } else {
                    used[cur] = true;
                }
                perm.push(1 << bit);
            }
        }
        for i in 0..n {
            if !used[i] {
                perm.push(i);
            }
        }
        perm.reverse();
        if ok {
            out.println("YES");
            out.println(perm);
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
    const PROBLEM_NAME: &str = "c_mexor";
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
