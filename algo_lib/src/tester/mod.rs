use std::io::{Cursor, Write};
use std::panic::UnwindSafe;
use std::sync::mpsc::Sender;
use std::time::Instant;

use crate::io::input::Input;
use crate::io::output::Output;

const EPS: f64 = 1e-9;

fn is_equal_floats(f_actual: f64, f_expected: f64) -> bool {
    let abs_diff = (f_actual - f_expected).abs();
    abs_diff <= EPS || abs_diff <= f_expected.abs() * EPS
}

fn is_equal_float_tokens(token_actual: Vec<u8>, token_expected: Vec<u8>) -> bool {
    if let Ok(f_actual) = String::from_utf8(token_actual).unwrap().parse::<f64>() {
        if let Ok(f_expected) = String::from_utf8(token_expected).unwrap().parse::<f64>() {
            return is_equal_floats(f_actual, f_expected);
        }
    }
    false
}

fn check(expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
    let mut expected = Input::new(Box::new(Cursor::new(expected.to_vec())));
    let mut actual = Input::new(Box::new(Cursor::new(actual.to_vec())));
    let mut token_num = 0usize;
    loop {
        let expected_token = expected.next_token();
        let actual_token = actual.next_token();
        if expected_token != actual_token {
            if expected_token.is_none() {
                return Err(format!("Expected has only {} tokens", token_num));
            } else if actual_token.is_none() {
                return Err(format!("Actual has only {} tokens", token_num));
            } else if !is_equal_float_tokens(
                actual_token.clone().unwrap(),
                expected_token.clone().unwrap(),
            ) {
                return Err(format!(
                    "Token #{} differs, expected {}, actual {}",
                    token_num,
                    String::from_utf8(expected_token.unwrap()).unwrap(),
                    String::from_utf8(actual_token.unwrap()).unwrap()
                ));
            }
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

struct WriteDelegate {
    snd: Sender<Vec<u8>>,
}

impl std::io::Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.snd.send(buf.to_vec()).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/**

Returns [true] in case of successes

 */
pub fn run_single_test(
    problem_name: &str,
    run: impl FnOnce(Input, Output) -> bool + UnwindSafe,
    test_name: &str,
) -> bool {
    let time_limit = std::time::Duration::from_millis(1000);
    let path = format!("./{problem_name}/tests/{test_name}.in");
    let out_path = format!("./{problem_name}/tests/{test_name}.out");
    println!("{}Test {}{}", BLUE, test_name, DEF);
    println!("{}Input:{}", BLUE, DEF);
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Can't open file with test input: {}", path));
    println!("{input}");
    let expected = match std::fs::read_to_string(out_path) {
        Ok(res) => Some(res),
        Err(_) => None,
    };
    println!("{}Expected:{}", BLUE, DEF);
    match &expected {
        None => {
            println!("{}Not provided{}", YELLOW, DEF);
        }
        Some(expected) => {
            println!("{}", expected);
        }
    }
    println!("{}Output:{}", BLUE, DEF);
    match std::panic::catch_unwind(|| {
        let input = Input::new_file(path);
        let (snd, rcv) = std::sync::mpsc::channel();
        let out: Box<dyn Write> = Box::new(WriteDelegate { snd });

        let output = Output::new(out);
        let started = std::time::Instant::now();
        let is_exhausted = run(input, output);
        let mut out_vec = Vec::new();
        while let Ok(buf) = rcv.recv() {
            out_vec.extend(buf);
        }
        let res = started.elapsed();
        println!("{}", String::from_utf8_lossy(&out_vec));
        (out_vec, res, is_exhausted)
    }) {
        Ok((output, duration, is_exhausted)) => {
            println!(
                "{}Time elapsed: {:.3}s{}",
                BLUE,
                (duration.as_millis() as f64) / 1000.,
                DEF,
            );
            if !is_exhausted {
                println!("{}Input not exhausted{}", RED, DEF);
            }
            if let Some(expected) = expected {
                let mut expected_bytes = expected.as_bytes().clone();
                match check(&mut expected_bytes, &mut &output[..]) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}Verdict: {}Wrong Answer ({}){}", BLUE, RED, err, DEF);
                        return false;
                    }
                }
            }
            if duration > time_limit {
                println!("{}Verdict: {}Time Limit{}", BLUE, RED, DEF);
                return false;
            } else {
                println!("{}Verdict: {}OK{}", BLUE, GREEN, DEF)
            }
        }
        Err(err) => {
            match err.downcast::<&str>() {
                Ok(as_string) => println!(
                    "{}Verdict: {}RuntimeError ({:?}){}",
                    BLUE, RED, as_string, DEF
                ),
                Err(err) => println!("{}Verdict: {}RuntimeError ({:?}){}", BLUE, RED, err, DEF),
            }
            return false;
        }
    }
    true
}

const BLUE: &str = "\x1B[34m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const DEF: &str = "\x1B[0m";

pub fn run_tests(
    problem_name: &str,
    run: impl FnOnce(Input, Output) -> bool + UnwindSafe + Clone,
) -> bool {
    let mut paths = std::fs::read_dir(format!("./{problem_name}/tests/"))
        .unwrap()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    paths.sort_by_key(|a| a.path());
    let mut test_failed = 0usize;
    let mut test_total = 0usize;
    for path in paths {
        let sub_path = path;
        if sub_path.file_type().unwrap().is_file() {
            let path = sub_path.path();
            match path.extension() {
                None => {}
                Some(extension) => {
                    if extension.to_str() == Some("in") {
                        println!("=====================================================");
                        test_total += 1;
                        let name = path.file_name().unwrap().to_str().unwrap();
                        let test_name = &name[..name.len() - 3];
                        if !run_single_test(problem_name, run.clone(), test_name) {
                            test_failed += 1;
                        }
                    }
                }
            }
        }
    }
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed{}",
            BLUE, GREEN, test_total, BLUE, DEF
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            RED, test_failed, test_total, BLUE, DEF
        );
    }
    test_failed == 0
}

pub fn run_locally(run: impl FnOnce(Input, Output) -> bool) {
    let input = Input::new_stdin();
    let output = Output::new_stdout();
    run(input, output);
}

pub fn run_stress(stress: fn() -> ()) {
    let start = Instant::now();
    stress();
    eprintln!("Finished in {}ms", start.elapsed().as_millis());
}
