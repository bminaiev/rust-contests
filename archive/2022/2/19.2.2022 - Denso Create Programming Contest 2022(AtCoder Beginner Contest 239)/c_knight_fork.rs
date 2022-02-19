//{"name":"C - Knight Fork","group":"AtCoder - Denso Create Programming Contest 2022(AtCoder Beginner Contest 239)","url":"https://atcoder.jp/contests/abc239/tasks/abc239_c","interactive":false,"timeLimit":2000,"tests":[{"input":"0 0 3 3\n","output":"Yes\n"},{"input":"0 1 2 3\n","output":"No\n"},{"input":"1000000000 1000000000 999999999 999999999\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKnightFork"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::Shift;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    type Point = PointT<i32>;
    let p1: Point = input.read();
    let p2: Point = input.read();
    const C: i32 = 10;
    for dx in -C..=C {
        for dy in -C..=C {
            let check = p1.apply_shift(&Shift { dx, dy });
            if check.dist2(&p1) == 5 && check.dist2(&p2) == 5 {
                out_line!("Yes");
                return;
            }
        }
    }
    out_line!("No");
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
}
//END MAIN
