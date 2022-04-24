//{"name":"B. Я люблю AAAB","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nAABAB\nABB\nAAAAAAAAB\nA\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BYaLyublyuAAAB"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let mut ok = *s.last_exn() == b'B';
    let mut cnta = 0;
    let mut cntb = 0;
    for &c in s.iter() {
        if c == b'A' {
            cnta += 1;
        } else {
            cntb += 1;
        }
        if cntb > cnta {
            ok = false;
        }
    }
    if ok {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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
