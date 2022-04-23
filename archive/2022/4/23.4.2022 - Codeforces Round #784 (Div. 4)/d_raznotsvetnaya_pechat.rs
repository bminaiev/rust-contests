//{"name":"D. Разноцветная печать","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"12\n5\nBRBBW\n1\nB\n2\nWB\n2\nRW\n3\nBRB\n3\nRBB\n7\nWWWWWWW\n9\nRBWBWRRBW\n10\nBRBRBRBRRB\n12\nBBBRWWRRRWBR\n10\nBRBRBRBRBW\n5\nRBWBW\n","output":"YES\nNO\nNO\nNO\nYES\nYES\nYES\nNO\nYES\nNO\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRaznotsvetnayaPechat"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn ok(s: &[u8]) -> bool {
    let mut i = 0;
    while i < s.len() {
        while i != s.len() && s[i] == b'W' {
            i += 1;
        }
        let mut j = i + 1;
        while j < s.len() && s[j] != b'W' {
            j += 1;
        }
        if i >= s.len() {
            break;
        }
        let len = j - i;
        if len == 1 {
            return false;
        }
        let mut ok = false;
        for pos in 0..(len - 1) {
            if s[i + pos] != s[i + pos + 1] {
                ok = true;
            }
        }
        if !ok {
            return false;
        }
        i = j;
    }
    true
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    if ok(&s) {
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
