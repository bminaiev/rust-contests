//{"name":"C. Автосинтез","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4 2 2 3\n","output":"3\n3 2 3\n"},{"input":"3\n1 2 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAvtosintez"}}}

use std::collections::VecDeque;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    UNKNOWN,
    RED,
    BLUE,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut inbound = vec![0; n];
    for i in 0..n {
        inbound[a[i]] += 1;
    }
    let mut colors = vec![Color::UNKNOWN; n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        if inbound[i] == 0 {
            queue.push_back(i);
            colors[i] = Color::RED;
        }
    }
    while let Some(v) = queue.pop_front() {
        let to = a[v];
        inbound[to] -= 1;
        if colors[v] == Color::RED {
            if colors[to] == Color::UNKNOWN {
                colors[to] = Color::BLUE;
                queue.push_back(to);
            }
        }
        if inbound[to] == 0 && colors[to] == Color::UNKNOWN {
            queue.push_back(to);
            colors[to] = Color::RED;
        }
    }
    for start in 0..n {
        if colors[start] == Color::UNKNOWN {
            colors[start] = Color::RED;
            let mut v = start;
            while colors[a[v]] == Color::UNKNOWN {
                if colors[v] == Color::RED {
                    colors[a[v]] = Color::BLUE;
                } else {
                    colors[a[v]] = Color::RED;
                }
                v = a[v];
            }
        }
    }
    let mut exist_inbound = vec![false; n];
    for v in 0..n {
        if colors[v] == Color::RED && colors[a[v]] == Color::RED {
            out_line!(-1);
            return;
        }
        if colors[v] == Color::RED {
            exist_inbound[a[v]] = true;
        }
    }
    for v in 0..n {
        if colors[v] == Color::BLUE && !exist_inbound[v] {
            out_line!(-1);
            return;
        }
    }
    let mut res = vec![];
    for v in 0..n {
        if colors[v] == Color::RED {
            res.push(a[v] + 1);
        }
    }
    out_line!(res.len());
    out_line!(res);
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
