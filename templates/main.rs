//$JSON

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
#[allow(unused)]
use algo_lib::{out, out_line, dbg};

$SOLVE

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
