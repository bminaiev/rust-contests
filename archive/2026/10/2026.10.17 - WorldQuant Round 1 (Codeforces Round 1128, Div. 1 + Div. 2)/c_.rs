use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _ in 0..n {
            a.push(input.i32() % 2);
        }
        a.sort();
        let mut first_win = true;
        if n == 2 && a[0] == 0 && a[1] == 1 {
            first_win = false;
        }
        if n == 3 && a[0] == 1 {
            first_win = false;
        }
        if first_win {
            out.println("Moggity");
        } else {
            out.println("Larpity");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

fn stress() {
    const N: usize = 10;
    let mut dp = Array2D::new(false, N, N);
    for cnt_ones in 0..N {
        for cnt_zeros in 0..N {
            let mut can_win = false;
            if cnt_ones > 0 && !dp[cnt_ones - 1][cnt_zeros] {
                can_win = true;
            }
            if cnt_zeros > 0 && !dp[cnt_ones][cnt_zeros - 1] {
                can_win = true;
            }
            for u_ones in 0..=cnt_ones {
                for u_zeros in 0..=cnt_zeros {
                    if u_ones + u_zeros > 0 {
                        if (u_ones % 2) == 0 {
                            if !dp[cnt_ones - u_ones][cnt_zeros - u_zeros] {
                                can_win = true;
                            }
                        }
                    }
                }
            }
            if !can_win {
                dbg!(cnt_ones, cnt_zeros);
            }
            dp[cnt_ones][cnt_zeros] = can_win;
        }
    }
    dbg!(dp[0][0]);
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_";
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
