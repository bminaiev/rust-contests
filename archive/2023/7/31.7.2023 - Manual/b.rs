//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::expr_eval::eval_expression;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn find_all_sols(s: &[u8], expected_res: i64) -> Vec<Vec<u8>> {
    let mut res = vec![];
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    let check = vec![a, b, c, d];
                    let test: Vec<_> = s
                        .iter()
                        .map(|&c| {
                            if c >= b'a' && c <= b'd' {
                                check[(c - b'a') as usize] + b'0'
                            } else {
                                c
                            }
                        })
                        .collect();
                    if eval_expression(&test) == expected_res {
                        res.push(check);
                    }
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let expected_res = input.i64();
    let sols = find_all_sols(&s, expected_res);
    out_line!(sols.len());
    if sols.len() == 1 {
        for x in sols[0].iter() {
            out!(*x);
        }
        out_line!();
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
