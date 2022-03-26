//{"name":"H. Prince and Princess","group":"Codeforces - The 2019 ICPC Asia Nanjing Regional Contest","url":"https://codeforces.com/gym/103466/problem/H","interactive":false,"timeLimit":1000,"tests":[{"input":"2 0 0\n","output":"YES\n1\n"},{"input":"1 1 0\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HPrinceAndPrincess"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_one(a: i32, b: i32, c: i32) -> Option<i32> {
    let n = a + b + c;
    if n == 1 {
        return Some(0);
    }
    if a <= b + c {
        return None;
    }

    Some((b + c) * 2 + 1)
}

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.i32();
    let b = input.i32();
    let c = input.i32();
    if let Some(r) = solve_one(a, b, c) {
        out_line!("YES");
        out_line!(r);
    } else {
        out_line!("NO");
    }
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
