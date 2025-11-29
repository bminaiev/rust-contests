//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn first_wins(s: &[u8], k: usize) -> bool {
    // first wants non-correct
    if s.len() % 2 == 1 || k % 2 == 1 {
        return true;
    }
    let mut my_close = vec![0; s.len()];
    let mut my_open = vec![0; s.len()];
    let mut opens = vec![];
    for i in 0..s.len() {
        let c = s[i];
        if c == b'(' {
            opens.push(i);
        } else {
            if opens.len() == 0 {
                return true;
            }
            let last_open = opens.pop().unwrap();
            my_close[last_open] = i;
            my_open[i] = last_open;
        }
    }
    if opens.len() > 0 {
        return true;
    }
    {
        let mut from = 0;
        let mut to = s.len();
        for _ in 0..(s.len() - k) / 2 {
            let cl = my_close[from];
            if cl == from + 1 {
                from += 2;
            } else if cl == to - 1 {
                from += 1;
                to -= 1;
            } else {
                return true;
            }
        }
    }
    {
        let mut from = 0;
        let mut to = s.len();
        for _ in 0..(s.len() - k) / 2 {
            let op = my_open[to - 1];
            if op == to - 2 {
                to -= 2;
            } else if op == from {
                from += 1;
                to -= 1;
            } else {
                return true;
            }
        }
    }
    false
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let s = input.string();
        let k = input.usize();
        if first_wins(&s, k) {
            out.println("First");
        } else {
            out.println("Second");
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
    const PROBLEM_NAME: &str = "a";
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
