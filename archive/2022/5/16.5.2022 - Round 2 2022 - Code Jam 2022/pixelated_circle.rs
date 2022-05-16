//{"name":"Pixelated Circle","group":"Google Coding Competitions - Round 2 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008778ec/0000000000b158f7","interactive":false,"timeLimit":10000,"tests":[{"input":"3\n2\n8\n50\n","output":"Case #1: 4\nCase #2: 24\nCase #3: 812\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PixelatedCircle"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn exist_good_r_specific(x: i32, y: i32, max_r: i32) -> bool {
    for r in x..=max_r {
        let f = r * r - x * x;
        let cur_y = ((f as f64).sqrt() + 0.5 + 1e-12) as i32;
        if cur_y == y {
            return true;
        }
    }
    false
}

fn exist_good_r(x: i32, y: i32, max_r: i32) -> bool {
    exist_good_r_specific(x.abs(), y.abs(), max_r) || exist_good_r_specific(y.abs(), x.abs(), max_r)
}

fn solve_r(r: i32) -> i32 {
    let mut res = 0;
    for x in -r..=r {
        for y in -r..=r {
            if x * x + y * y <= r * r + r {
                if !exist_good_r(x, y, r) {
                    if x >= 0 && y >= x {
                        // dbg!(x, y);
                    }
                    res += 1;
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, test_case: usize) {
    let r = input.i32();

    out_line!(format!("Case #{}: {}", test_case, solve_r(r)));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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

fn stress() {
    for r in 1..=100 {
        dbg!(r, solve_r(r));
    }
}

fn main() {
    tester::run_tests();
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
