//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    w: i32,
    x: i32,
    y: i32,
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut a = vec![];
    let mut b = vec![];
    let mut needed = vec![];
    let q = input.usize();
    let mut last_res = 0;
    for _ in 0..q {
        let q_type = input.i32();
        if q_type == 1 {
            let p = Point {
                x: input.i32() ^ last_res,
                y: input.i32() ^ last_res,
                w: input.i32() ^ last_res,
            };
            a.push(p);
            a.sort();
            a.reverse();
            needed.clear();
            for _ in 0..a.len() {
                needed.push(false);
            }
            for x_i in 0..=a.len() {
                let x = if x_i == a.len() {
                    std::i32::MAX
                } else {
                    a[x_i].x
                };
                for y_i in 0..=a.len() {
                    let y = if y_i == a.len() {
                        std::i32::MAX
                    } else {
                        a[y_i].y
                    };
                    for i in 0..a.len() {
                        if a[i].x != x && a[i].y != y {
                            needed[i] = true;
                            break;
                        }
                    }
                }
            }
            b.clear();
            for i in 0..a.len() {
                if needed[i] {
                    b.push(a[i]);
                }
            }
            a.clear();
            for p in b.iter() {
                a.push(*p);
            }
        } else {
            assert_eq!(q_type, 2);
            let p = Point {
                x: input.i32() ^ last_res,
                y: input.i32() ^ last_res,
                w: 0,
            };
            last_res = 0;
            for check_p in a.iter() {
                if check_p.x != p.x && check_p.y != p.y {
                    last_res = check_p.w;
                    break;
                }
            }
            out_line!(last_res);
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
