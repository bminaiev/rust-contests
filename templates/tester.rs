use std::io::Cursor;

use algo_lib::io::input::Input;
use algo_lib::io::output::{Output, OUTPUT, set_global_output_to_stdout};

const EPS: f64 = 1e-9;

fn is_equal_floats(f_actual: f64, f_expected: f64) -> bool {
    let abs_diff = (f_actual - f_expected).abs();
    return abs_diff <= EPS || abs_diff <= f_expected.abs() * EPS;
}

fn is_equal_float_tokens(token_actual: Vec<u8>, token_expected: Vec<u8>) -> bool {
    if let Ok(f_actual) = String::from_utf8(token_actual).unwrap().parse::<f64>() {
        if let Ok(f_expected) = String::from_utf8(token_expected).unwrap().parse::<f64>() {
            return is_equal_floats(f_actual, f_expected);
        }
    }
    return false;
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
            } else {
                if !is_equal_float_tokens(
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
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

static mut OUT: Vec<u8> = Vec::new();

struct WriteDelegate {}

impl std::io::Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            OUT.append(&mut Vec::from(buf));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/**

Returns [true] in case of successes

 */
pub(crate) fn run_single_test(name: &str) -> bool {
    let time_limit = std::time::Duration::from_millis($TIME_LIMIT);
    let path = format!("./$TASK/tests/{}.in", name);
    let out_path = format!("./$TASK/tests/{}.out", name);
    println!("{}Test {}{}", BLUE, name, DEF);
    println!("{}Input:{}", BLUE, DEF);
    println!(
        "{}",
        std::fs::read_to_string(&path)
            .expect(&format!("Can't open file with test input: {}", path))
    );
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
        unsafe {
            OUT.clear();
        }
        let input = Input::new_file(path);
        let started = std::time::Instant::now();
        unsafe {
            OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
        }
        let is_exhausted = crate::run(input);
        let res = started.elapsed();
        let output;
        unsafe {
            output = OUT.clone();
        }
        println!("{}", String::from_utf8_lossy(&output));
        (output, res, is_exhausted)
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
                Err(err) => println!(
                    "{}Verdict: {}RuntimeError ({:?}){}",
                    BLUE, RED, err, DEF
                ),
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

#[allow(unused)]
pub(crate) fn run_tests() -> bool {
    let mut paths = std::fs::read_dir("./$TASK/tests/")
        .unwrap()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    paths.sort_by(|a, b| a.path().cmp(&b.path()));
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
                        let name = &name[..name.len() - 3];
                        if !run_single_test(name) {
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

#[allow(unused)]
pub fn run_locally() {
    let sin = std::io::stdin();
    let input = Input::new(Box::new(sin));
    set_global_output_to_stdout();
    crate::run(input);
}