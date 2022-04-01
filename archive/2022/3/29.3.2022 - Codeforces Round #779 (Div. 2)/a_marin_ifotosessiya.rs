//{"name":"A. Марин и фотосессия","group":"Codeforces - Codeforces Round #779 (Div. 2)","url":"https://codeforces.com/contest/1658/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n3\n000\n3\n001\n3\n010\n3\n011\n3\n100\n3\n101\n3\n110\n3\n111\n19\n1010110000100000101\n","output":"4\n2\n1\n0\n2\n0\n0\n0\n17\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMarinIFotosessiya"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut cur = vec![];
    for &c in s.iter() {
        if c == b'0' {
            if cur.len() >= 1 && *cur.last_exn() == b'0' {
                cur.push(b'1');
                cur.push(b'1');
            } else if cur.len() >= 2 && *cur.last_exn() == b'1' && cur[cur.len() - 2] == b'0' {
                cur.push(b'1');
            }
        }
        cur.push(c);
    }
    out_line!(cur.len() - n);
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
