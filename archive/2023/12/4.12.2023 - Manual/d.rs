//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn conv(c: u8) -> i32 {
    if c >= b'0' && c <= b'9' {
        return (c - b'0') as i32;
    }
    if c >= b'a' && c <= b'z' {
        return (c - b'a') as i32 + 10;
    }
    if c >= b'A' && c <= b'Z' {
        return (c - b'A') as i32 + 36;
    }
    panic!();
}

fn ok(a: i32, b: i32, c: i32) -> bool {
    a + c == b * 2
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| {
        let s = input.string();
        let mut res = 0;
        for c in s.iter() {
            res = res * 62 + conv(*c);
        }
        res
    });
    let mut k = 1;
    for len in 1..=n {
        if len >= 2 * k + 1 && ok(a[len - 2 * k - 1], a[len - k - 1], a[len - 1]) {
            out!("1");
        } else {
            while k * 2 + 1 <= len {
                let mut okk = true;
                for start in (0..len - 2 * k).rev() {
                    if !ok(a[start], a[start + k], a[start + 2 * k]) {
                        okk = false;
                        break;
                    }
                }
                if okk {
                    break;
                } else {
                    k += 1;
                }
            }
            if k * 2 >= len {
                out!("0");
            } else {
                out!("1");
            }
        }
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

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
