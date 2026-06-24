#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy)]
enum Event {
    Add(usize),
    Remove(usize),
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let s = input.string();
        let mut cnt_zeros = 0;
        for i in 0..n {
            if s[i] == b'0' {
                cnt_zeros += 1;
            } else {
                break;
            }
        }
        if cnt_zeros + 1 < k || (k == 1 && cnt_zeros > 0) {
            out.println(-1);
        } else {
            let mut events = vec![];
            let extra = cnt_zeros - (k - 1);
            for i in 0..extra {
                events.push(Event::Add(i));
                events.push(Event::Remove(i));
            }
            for i in extra..cnt_zeros {
                events.push(Event::Add(i));
            }
            let mut last = None;
            for i in cnt_zeros..n {
                if s[i] == b'0' {
                    events.push(Event::Add(i));
                    events.push(Event::Remove(i));
                } else {
                    events.push(Event::Add(i));
                    if let Some(last) = last {
                        events.push(Event::Remove(last));
                    }
                    last = Some(i);
                }
            }
            out.println(events.len());
            for e in events.iter() {
                match e {
                    Event::Add(i) => out.println(format!("I {}", i + 1)),
                    Event::Remove(i) => out.println(format!("O {}", i + 1)),
                }
            }
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
    const PROBLEM_NAME: &str = "k";
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
