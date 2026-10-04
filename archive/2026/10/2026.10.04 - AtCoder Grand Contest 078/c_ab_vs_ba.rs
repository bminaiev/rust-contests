#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

// LOL. The best problem I have seen...

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut s = input.string();
        let mut cnt_as = 0;
        for i in 0..n {
            if s[i] == b'A' {
                cnt_as += 1;
            }
        }
        if cnt_as * 2 < n {
            s.reverse();
            for i in 0..n {
                if s[i] == b'A' {
                    s[i] = b'B';
                } else {
                    s[i] = b'A';
                }
            }
            cnt_as = n - cnt_as;
        }
        let cnt_bs = n - cnt_as;
        let s_rev = {
            let mut s_rev = s.clone();
            s_rev.reverse();
            s_rev
        };
        let v = calc(&s_rev) - calc(&s);
        let alice_wins = if v > 0 {
            true
        } else if v < 0 {
            false
        } else {
            cnt_bs % 2 == 1
        };
        if alice_wins {
            out.println("Abel");
        } else {
            out.println("Bart");
        }
    }
}

fn calc(s: &[u8]) -> i32 {
    let mut h = 0i32;
    let mut largest_h = 0;
    let mut records = 0;
    for i in 0..s.len() {
        if s[i] == b'A' {
            h += 1;
        } else {
            h -= 1;
        }
        if h.abs() > largest_h {
            largest_h = h.abs();
            if h < 0 {
                records += 1;
            }
        }
    }
    records
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_ab_vs_ba";
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
