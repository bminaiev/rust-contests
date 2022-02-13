//{"name":"B. Алфавитные строки","group":"Codeforces - Codeforces Round #731 (Div. 3)","url":"https://codeforces.com/contest/1547/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"11\na\nba\nab\nbac\nihfcbadeg\nz\naa\nca\nacb\nxyz\nddcba\n","output":"YES\nYES\nYES\nYES\nYES\nNO\nNO\nNO\nNO\nNO\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAlfavitnieStroki"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn ok(s: &[u8]) -> bool {
    let len = s.len();
    if len == 0 {
        return true;
    }
    let expect = b'a' + (len - 1) as u8;
    if s[0] == expect {
        return ok(&s[1..]);
    } else if s[len - 1] == expect {
        return ok(&s[0..len - 1]);
    }
    false
}

fn solve(input: &mut Input, _test_case: usize) {
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
