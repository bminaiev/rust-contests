//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn get_s(s: &[u8]) -> Option<Vec<u8>> {
    for i in 1..s.len() {
        if s[i] == b',' {
            return Some(s[..i].to_vec());
        }
    }
    None
}

fn expect_first(s: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    res.extend(s.to_vec());
    res.push(b',');
    res.push(b' ');
    res.extend(s.to_vec());
    res.push(b' ');
    res.extend(s.to_vec());
    res.push(b'i');
    res.push(b't');
    res.push(b'y');
    res.push(b' ');
    res.extend(s.to_vec());
    res
}

fn expect_second(s: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    res.push(b'i');
    res.push(b' ');
    res.push(b's');
    res.push(b'a');
    res.push(b'i');
    res.push(b'd');
    res.push(b' ');
    res.extend(s.to_vec());
    res.push(b',');
    res.push(b' ');
    res.extend(s.to_vec());
    // res.push(b',');
    res.push(b' ');
    res.extend(s.to_vec());
    res.push(b'i');
    res.push(b't');
    res.push(b'y');
    res.push(b' ');
    res.extend(s.to_vec());
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let mut lines = vec![];
    while input.has_more_elements() {
        lines.push(input.read_line().as_bytes().to_vec());
    }
    let mut res = 0;
    let mut i = 0;
    while i < lines.len() {
        if let Some(s) = get_s(&lines[i]) {
            let first = expect_first(&s);
            let second = expect_second(&s);
            // let s_str = String::from_utf8_lossy(&s);
            // let first_str = String::from_utf8_lossy(&first);
            // let second_str = String::from_utf8_lossy(&second);
            // dbg!(i, &s_str, first_str, second_str);
            let mut j = i;
            while j + 2 <= lines.len() {
                if lines[j] == first && lines[j + 1] == second {
                    j += 2;
                } else {
                    break;
                }
            }
            let cur_len = (first.len() + second.len() + 2) * ((j - i) / 2);
            res = res.max(cur_len);
            if i == j {
                i += 1;
            } else {
                i = j;
            }
        } else {
            i += 1;
        }
    }

    if res == 0 {
        out.println(-1);
    } else {
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "g";
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
