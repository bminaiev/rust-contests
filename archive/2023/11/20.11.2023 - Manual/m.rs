//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
enum Op {
    Circle {
        x: i64,
        y: i64,
        r: i64,
        col: u8,
    },
    Rectangle {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        col: u8,
    },
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut ops = vec![];
    for _ in 0..n {
        let q_type = input.string_as_string();
        if q_type == "Circle" {
            let x = input.i64();
            let y = input.i64();
            let r = input.i64();
            let col = input.string()[0];
            ops.push(Op::Circle { x, y, r, col });
        } else if q_type == "Rectangle" {
            let x1 = input.i64();
            let y1 = input.i64();
            let x2 = input.i64();
            let y2 = input.i64();
            let col = input.string()[0];
            ops.push(Op::Rectangle {
                x1,
                y1,
                x2,
                y2,
                col,
            });
        } else {
            assert_eq!(q_type, "Render");
            let x1 = input.i64();
            let y1 = input.i64();
            let x2 = input.i64();
            let y2 = input.i64();
            for cy in (y1..=y2).rev() {
                for cx in x1..=x2 {
                    let mut res = b'.';
                    for op in ops.iter() {
                        match op {
                            &Op::Circle { x, y, r, col } => {
                                if (cx - x) * (cx - x) + (cy - y) * (cy - y) <= r * r {
                                    res = col;
                                }
                            }
                            &Op::Rectangle {
                                x1,
                                y1,
                                x2,
                                y2,
                                col,
                            } => {
                                if cx >= x1 && cx <= x2 && cy >= y1 && cy <= y2 {
                                    res = col;
                                }
                            }
                        }
                    }
                    out!(res as char);
                }
                out_line!()
            }
        }
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
