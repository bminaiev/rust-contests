//{"name":"B. Докажи, что он не прав","group":"Codeforces - Educational Codeforces Round 124 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1651/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n512\n3\n","output":"YES\n1 337\nNO\nYES\n31 4 159\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDokazhiChtoOnNePrav"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        if n > 19 {
            out_line!("NO");
            continue;
        } else {
            out_line!("YES");
            let mut x = 1i64;
            for _ in 0..n {
                out!(x, "");
                assert!(x <= 1_000_000_000);
                x *= 3;
            }
            out_line!();
        }
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
    // tester::run_locally();
}
//END MAIN
