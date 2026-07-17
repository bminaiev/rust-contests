use std::collections::HashSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn sum_digits(mut n: i128) -> i128 {
    let mut res = 0;
    while n > 0 {
        res += n % 10;
        n /= 10;
    }
    res
}

fn solve_case(d: i128) -> Option<(i128, i128)> {
    // recusrive function:
    // pos, a, b
    let mut seen = HashSet::new();

    RecursiveFunction3::new(|f, pos: usize, a: i128, b: i128| -> Option<(i128, i128)> {
        if pos == 33 {
            return None;
        }
        let sum_a = sum_digits(a);
        let sum_b = sum_digits(b);
        if sum_a == sum_b && a - b == d {
            return Some((a, b));
        }
        let pw10 = 10i128.pow(pos as u32);
        if ((a - b - d) % pw10) != 0 {
            return None;
        }
        let key = (pos, sum_a - sum_b, a - b);
        if !seen.insert(key) {
            return None;
        }
        for x in 0..10 {
            for y in 0..10 {
                let na = a + x * pw10;
                let nb = b + y * pw10;
                if let Some(res) = f.call(pos + 1, na, nb) {
                    return Some(res);
                }
            }
        }

        None
    })
    .call(0, 0, 0)
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let ds = input.vec::<i128>(tc);
    let res = ds
        .par_iter()
        .map(|&d| (d, solve_case(d)))
        .collect::<Vec<_>>();
    let mut cnt_none = 0;
    for (d, res) in res {
        if let Some((x, y)) = res {
            assert!(x - y == d);
            out.println(vec![x, y]);
        } else {
            cnt_none += 1;
            out.println("NONE");
        }
    }
    dbg!(cnt_none);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "n";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    run_dragon(solve, "N", 1..4);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
