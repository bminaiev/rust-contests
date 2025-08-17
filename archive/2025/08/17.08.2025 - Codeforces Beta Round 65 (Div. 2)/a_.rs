//{"name":"A. Слишком длинные слова","group":"Codeforces - Codeforces Beta Round 65 (Div. 2)","url":"https://codeforces.com/problemset/problem/71/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nword\nlocalization\ninternationalization\npneumonoultramicroscopicsilicovolcanoconiosis\n","output":"word\nl10n\ni18n\np43s\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    for _ in 0..n {
        let s = input.string();
        let len = s.len();
        if len > 10 {
            let first = s[0] as char;
            let last = s[len - 1] as char;
            out.println(format!("{}{}{}", first, len - 2, last));
        } else {
            out.println(String::from_utf8(s).unwrap());
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[allow(dead_code)]
fn stress() {
    use std::io::Cursor;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..101);
        let mut input_str = String::new();
        input_str.push_str(&format!("{}\n", n));
        for _ in 0..n {
            let len = rnd.gen(1..101);
            let word: String = (0..len)
                .map(|_| (b'a' + rnd.gen(0..26) as u8) as char)
                .collect();
            input_str.push_str(&word);
            input_str.push('\n');
        }
        let input = Input::new(Box::new(Cursor::new(input_str.into_bytes())));
        let output = Output::new(Box::new(std::io::sink()));
        run(input, output);
    }
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
