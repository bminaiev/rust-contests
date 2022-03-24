//{"name":"E. Escalator","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/E","interactive":false,"timeLimit":500,"tests":[{"input":"3\n5 0\n8 0\n13 0\n","output":"23\n"},{"input":"3\n5 0\n7 1\n9 0\n","output":"29\n"},{"input":"3\n5 0\n10 1\n16 0\n","output":"35\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEscalator"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut cur_dir = 0;
    let mut finish_at = 0;
    let mut should_go_opposite_dir = false;
    for _ in 0..n {
        let now = input.i32();
        let dir = input.i32();
        if now < finish_at && dir == cur_dir {
            finish_at = now + 10;
        } else if now >= finish_at && dir != cur_dir {
            finish_at = now + 10;
            cur_dir = dir;
            should_go_opposite_dir = false;
        } else if dir != cur_dir && now < finish_at {
            should_go_opposite_dir = true;
        } else if dir == cur_dir && now > finish_at {
            if should_go_opposite_dir {
                finish_at += 10;
                if now > finish_at {
                    should_go_opposite_dir = false;
                    finish_at = now + 10;
                } else {
                    cur_dir = 1 - cur_dir;
                    should_go_opposite_dir = true;
                }
            } else {
                finish_at = now + 10;
            }
        }
    }
    if should_go_opposite_dir {
        finish_at += 10;
    }
    out_line!(finish_at);
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
}
//END MAIN
