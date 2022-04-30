//{"name":"A. Серия преступлений","group":"Codeforces - Чемпионат КРОК 2012 - Раунд 2 (неофиц. редакция для Div. 2)","url":"https://codeforces.com/contest/181/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n.*\n..\n**\n","output":"1 1\n"},{"input":"3 3\n*.*\n*..\n...\n","output":"2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASeriyaPrestuplenii"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let s = gen_vec(n, |_| input.string());
    let mut rx = 0;
    let mut ry = 0;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == b'*' {
                rx ^= i;
                ry ^= j;
            }
        }
    }
    out_line!(rx + 1, ry + 1);
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
