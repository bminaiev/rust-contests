//$JSON

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

$SOLVE

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "$TASK";
    use algo_lib::tester::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN