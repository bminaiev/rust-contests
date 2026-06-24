#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::gcd::gcd;

fn solve(input: &mut Input, out: &mut Output) {}

fn stress() {
    const MX: usize = 900;
    let mut seen = vec![false; MX * MX * MX * 2 + 1];
    for x in 1..=MX {
        for y in x..=MX {
            let value = x * x * x + y * y * y;
            if value < seen.len() {
                seen[value] = true;
            }
        }
    }
    for mx in 1..5090 {
        dbg!(mx);
        for another in 1..=mx {
            for is_p_max in [false, true] {
                let p = if is_p_max { mx } else { another };
                let q = if is_p_max { another } else { mx };
                let g = gcd(p, q);
                if g > 1 {
                    continue;
                }
                for k in 1.. {
                    let pp = (p * k) as usize;
                    let qq = (q * k) as usize;
                    if pp >= seen.len() || qq >= seen.len() {
                        dbg!("FAIL", p, q);
                        break;
                    }
                    if seen[pp] && seen[qq] {
                        break;
                    }
                }
            }
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
    const PROBLEM_NAME: &str = "b";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
