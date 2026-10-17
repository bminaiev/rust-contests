use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.i64();
    let mut a = input.vec::<i64>(n);
    let sum: i64 = a.iter().sum();
    let part = sum / m;
    let mut max_seen = 0;
    let mut rnd = Random::new_time_seed();
    let start = Instant::now();
    while start.elapsed().as_millis() < 1900 {
        rnd.shuffle(&mut a);
        let mut seen = 0;
        let mut cur_sum = 0;
        for &x in a.iter() {
            cur_sum += x;
            if cur_sum % part == 0 {
                seen += 1;
            }
        }
        max_seen = seen.max(max_seen);
    }
    out.println(m - max_seen);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f_cake_cutting";
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
