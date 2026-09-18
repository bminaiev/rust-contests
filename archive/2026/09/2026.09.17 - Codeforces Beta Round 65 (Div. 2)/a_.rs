//{"name":"A. Слишком длинные слова","group":"Codeforces - Codeforces Beta Round 65 (Div. 2)","url":"https://codeforces.com/problemset/problem/71/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nword\nlocalization\ninternationalization\npneumonoultramicroscopicsilicovolcanoconiosis\n","output":"word\nl10n\ni18n\np43s\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}



#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
let input = algo_lib::io::input::Input::new_stdin();
let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
