//$JSON

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

$SOLVE


#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "$TASK";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
$INPUT
$OUTPUT
    run(input, output);
}