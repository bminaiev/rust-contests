#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let read_str = |input: &mut Input| -> Vec<usize> {
        let s = input.string();
        let mut res = vec![];
        for i in 0..s.len() {
            if s[i] == b'1' {
                res.push(i);
            }
        }
        res
    };
    let a = read_str(input);
    let b = read_str(input);
    let c = read_str(input);
    let mut positions = vec![];
    let mut res = 0;
    for i in 0..n {
        let mut tmp = [a[i], b[i], c[i]];
        tmp.sort();
        positions.push(tmp[1]);
        res += tmp[2] - tmp[0];
    }
    for i in 0..n - 1 {
        assert!(positions[i] < positions[i + 1]);
    }
    let mut answer = vec![b'0'; n * 2];
    for i in 0..n {
        answer[positions[i]] = b'1';
    }
    out.println(res);
    out.println(String::from_utf8(answer).unwrap());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_fermat_point_of_binary_strings";
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
