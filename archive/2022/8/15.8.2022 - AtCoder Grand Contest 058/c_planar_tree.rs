//{"name":"C - Planar Tree","group":"AtCoder - AtCoder Grand Contest 058","url":"https://atcoder.jp/contests/agc058/tasks/agc058_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n1 2 3 4\n4\n1 3 4 2\n4\n1 4 2 3\n","output":"Yes\nYes\nNo\n"},{"input":"3\n8\n4 2 3 4 1 2 2 1\n8\n3 2 2 3 1 3 3 4\n8\n2 3 3 2 1 4 1 4\n","output":"Yes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPlanarTree"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let tc = input.usize();

    let mut replacements = vec![];
    for x in 1..=4 {
        replacements.push((vec![x, x], vec![x]));
    }
    for &x in [2, 3].iter() {
        let another = if x == 2 { 1 } else { 4 };
        replacements.push((vec![x, another], vec![x]));
        replacements.push((vec![another, x], vec![x]));
        let can_be_removed = 5 - x;
        replacements.push((
            vec![another, can_be_removed, x, can_be_removed],
            vec![x, can_be_removed],
        ));
        replacements.push((
            vec![can_be_removed, x, can_be_removed, another],
            vec![can_be_removed, x],
        ));
    }

    for _ in 0..tc {
        let n = input.usize();
        let mut a = input.vec::<usize>(n);
        let mut rnd = Random::new(333);
        let mut iters = 0;
        while iters < 30 && a.len() > 0 {
            iters += 1;
            let shift = rnd.gen(0..a.len());
            let mut stack = vec![];
            for i in 0..a.len() {
                let x = a[(i + shift) % a.len()];
                stack.push(x);
                loop {
                    let mut changed = false;
                    for (check, rep) in replacements.iter() {
                        if stack.ends_with(check) {
                            for _ in 0..check.len() {
                                stack.pop();
                            }
                            stack.extend(rep);
                            changed = true;
                            iters = 0;
                        }
                    }
                    if !changed {
                        break;
                    }
                }
            }
            a = stack;
        }
        while !a.is_empty() && *a.last_exn() != 1 && *a.last_exn() != 4 {
            a.pop();
        }

        if a.is_empty() {
            out_line!("Yes");
        } else {
            out_line!("No");
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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
}
//END MAIN
