//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CharOfNumber {
    Number(i64),
    Char(u8),
}

fn eval(s: &[CharOfNumber]) -> i64 {
    // assert!(!s.contains(&b'('));
    // assert!(!s.contains(&b')'));
    let mut ops = vec![];
    let mut numbers = vec![];
    let mut cur_number = 0;
    for i in 0..s.len() {
        match s[i] {
            CharOfNumber::Number(x) => {
                assert_eq!(cur_number, 0);
                cur_number = x;
            }
            CharOfNumber::Char(c) => {
                if c == b'+' || c == b'*' || c == b'-' {
                    ops.push(c);
                    numbers.push(cur_number);
                    cur_number = 0;
                } else {
                    cur_number = cur_number * 10 + (c - b'0') as i64;
                }
            }
        }
    }
    numbers.push(cur_number);
    let mut new_numbers = vec![numbers[0]];
    let mut new_ops = vec![];
    for i in 0..ops.len() {
        if ops[i] == b'*' {
            let last = new_numbers.pop().unwrap();
            new_numbers.push(last * numbers[i + 1]);
        } else {
            new_ops.push(ops[i]);
            new_numbers.push(numbers[i + 1]);
        }
    }
    let mut res = new_numbers[0];
    for i in 0..new_ops.len() {
        if new_ops[i] == b'+' {
            res += new_numbers[i + 1];
        } else {
            res -= new_numbers[i + 1];
        }
    }
    res
}

fn eval_rec(s: &[CharOfNumber]) -> i64 {
    for i in 0..s.len() {
        if s[i] == CharOfNumber::Char(b'(') {
            let mut cnt = 1;
            for j in i + 1.. {
                if s[j] == CharOfNumber::Char(b'(') {
                    cnt += 1;
                }
                if s[j] == CharOfNumber::Char(b')') {
                    cnt -= 1;
                }
                if cnt == 0 {
                    let res = eval_rec(&s[i + 1..j]);
                    let mut new_s = vec![];
                    for k in 0..i {
                        new_s.push(s[k]);
                    }
                    new_s.push(CharOfNumber::Number(res));
                    for k in j + 1..s.len() {
                        new_s.push(s[k]);
                    }
                    return eval_rec(&new_s);
                }
            }
        }
    }
    eval(s)
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let expected = input.i64();
    let mut res = vec![];
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    let mut to_check = vec![];
                    let cur_way = vec![a, b, c, d];
                    for &x in s.iter() {
                        if x >= b'a' && x <= b'd' {
                            let number = cur_way[x as usize - b'a' as usize] + b'0';
                            to_check.push(CharOfNumber::Char(number));
                        } else {
                            to_check.push(CharOfNumber::Char(x));
                        }
                    }
                    if eval_rec(&to_check) == expected {
                        res.push(cur_way);
                    }
                }
            }
        }
    }
    out_line!(res.len());
    if res.len() == 1 {
        out_line!(res[0]
            .iter()
            .map(|&x| (x + b'0') as char)
            .collect::<String>());
    }
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
