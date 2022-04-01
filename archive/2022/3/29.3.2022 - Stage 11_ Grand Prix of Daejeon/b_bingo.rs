//{"name":"B. Bingo","group":"Yandex - Stage 11: Grand Prix of Daejeon","url":"https://official.contest.yandex.com/opencupXXII/contest/35265/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2\n","output":"YES\n##..\n....\n....\n....\n"},{"input":"4 16\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBingo"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn gen(n: usize) -> Array2D<bool> {
    let mut res = Array2D::new(false, n, n);
    res[0][0] = true;
    res[n - 1][n - 1] = true;
    for i in 1..n - 1 {
        res[i][n - 1 - i] = true;
    }
    if n == 2 {
        res[0][1] = true;
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut need = input.usize();
    let mut res = gen(n);
    for i in 0..n {
        for j in 0..n {
            if !res[i][j] {
                if need != 0 {
                    need -= 1;
                } else {
                    res[i][j] = true;
                }
            }
        }
    }
    if need == 0 {
        out_line!("YES");
        for i in 0..n {
            for j in 0..n {
                if res[i][j] {
                    out!(".");
                } else {
                    out!("#");
                }
            }
            out_line!();
        }
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
