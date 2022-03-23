//{"name":"petr_10_g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"rle-size.in","pattern":null},"output":{"type":"file","fileName":"rle-size.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_g"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn min_cnt(s: &[u8]) -> usize {
    let mut res = 0;
    let mut last = b'.';
    for &c in s.iter() {
        if c == b'?' {
            continue;
        }
        if c != last {
            last = c;
            res += 1;
        }
    }
    if res <= 1 {
        1
    } else {
        res
    }
}

fn inv(x: u8) -> u8 {
    if x == b'+' {
        b'-'
    } else {
        b'+'
    }
}

fn max_cnt(s: &[u8]) -> usize {
    let mut cur: Vec<u8> = s.iter().cloned().collect();
    loop {
        let mut changed = false;
        for i in 0..s.len() - 1 {
            if cur[i] == b'?' && cur[i + 1] != b'?' {
                cur[i] = inv(cur[i + 1]);
                changed = true;
            }
            if cur[i] != b'?' && cur[i + 1] == b'?' {
                cur[i + 1] = inv(cur[i]);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    if cur[0] == b'?' {
        return cur.len();
    }
    let mut res = 1;
    for w in cur.windows(2) {
        if w[0] != w[1] {
            res += 1;
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    out_line!(min_cnt(&s), max_cnt(&s));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::File("rle-size.in".to_string()),
        output: TaskIoType::File("rle-size.out".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_locally();
}
//END MAIN
