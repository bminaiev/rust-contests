//{"name":"C. Получи четную строку","group":"Codeforces - Codeforces Round #780 (Div. 3)","url":"https://codeforces.com/contest/1660/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\naabbdabdccc\nzyx\naaababbb\naabbcc\noaoaaaoo\nbmefbmuyw\n","output":"3\n3\n2\n0\n2\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPoluchiChetnuyuStroku"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let mut any = vec![false; 26];
    let mut res = 0;
    for c in s.into_iter() {
        let x = (c - b'a') as usize;
        if any[x] {
            for y in any.iter_mut() {
                *y = false;
            }
        } else {
            if any.iter().any(|f| *f) {
                res += 1;
            }
            any[x] = true;
        }
    }
    if any.iter().any(|f| *f) {
        res += 1;
    }
    out_line!(res);
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

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
