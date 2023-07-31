//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_one(input: &mut Input, n: usize) {
    let mut idx = 0;
    for bit in 0..9 {
        let check = 1 << bit;
        if check < n {
            out_line!(check + 1);
            output().flush();
            let res = input.string_as_string();
            if res == "green" {
                return;
            }
            if res == "yellow" {
                idx |= check;
            } else {
                assert_eq!(res, "red");
            }
        }
    }
    assert!(idx < n);
    out_line!(idx + 1);
    output().flush();
    let res = input.string_as_string();
    assert_eq!(res, "green");
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut edges = vec![];
    for i in 0..n {
        for j in i + 1..n {
            if (i & j) != 0 {
                edges.push((i, j));
            }
        }
    }

    out_line!(edges.len());
    for (i, j) in edges {
        out_line!(i + 1, j + 1);
    }
    for _ in 0..q {
        solve_one(input, n);
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
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
